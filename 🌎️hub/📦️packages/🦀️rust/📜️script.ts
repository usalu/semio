#!/usr/bin/env bun
import { createHash, createHmac, randomBytes, timingSafeEqual, webcrypto } from "node:crypto";
import { chmodSync, closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeSync } from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { spawn, type ChildProcess } from "node:child_process";
import type { Duplex } from "node:stream";
import Ajv from "ajv";
import { decodeClientFrame, decodePresencePeer, encodePresencePeer, encodeServerFrame, type ArtifactPresencePeer, type WireFrontierSummary } from "../../../🧰️framework/🔨️modules/📡️replication/🟦️.ts";
import { decodeBackboneWorkerResponse, decodePackValue, encodeBackboneWorkerRequest, encodePackValue } from "../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, parseDocumentOpenIntentV1, parseDocumentOpenPlanV1, parseDocumentPlanSocketGrantIntentV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { directoryCommandErrorFromStatus, directoryCommandErrorIsTransient, directoryCommandRequestJson, directoryCommandSha256, parseDirectoryCommandReceiptV1, parseDirectoryCommandRequestV1, sealDirectoryCommandRequestV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import type { DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { produceFreshComponentV1, type FreshBuildControlV1, type FreshComponentReceiptV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts";
import { verifyFreshCatalogPackageV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts";
/** 🌎️ `os-hub` router: `bun ./📜️script.ts <setup|build|test|dev>`. */
import {
  BundleScript,
  ScriptRouter,
  OS_HUB_PORT,
  OS_HUB_PORT_ENV,
  runBundleScriptMain,
  runCargo,
  runCargoTestBudgeted,
  runExactCargoLaws,
  runCmd,
  runProbe,
  buildBudgetMs,
  orchestratorBudgetOpts,
  resolveTestLevel,
} from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const LOCAL_BOOTSTRAP_SCHEMA = "semio.hub.local-bootstrap/v1";
const LOCAL_BOOTSTRAP_DOMAIN = "semio/hub/local-bootstrap/v1\0";
const LOCAL_BOOTSTRAP_FRAME_MAX = 16 * 1024;
const LOCAL_BOOTSTRAP_DEADLINE_MS = 15_000;
const LOCAL_READINESS_DEADLINE_MS = 30_000;
type LocalClientClass = "native" | "mcp" | "react-relay" | "admin-relay";
type LocalProfile = { readonly profileId: string; readonly subject: string; readonly displayName: string; readonly allowedClientClasses: readonly LocalClientClass[] };
type AdminLiveJourneyFixture = {
  readonly schema: "semio.hub.admin-live-journey/v1";
  readonly profile: { readonly profileId: string; readonly subject: string; readonly displayName: string };
  readonly limits: { readonly journeyMs: number; readonly pollMs: number; readonly responseBytes: 65536 };
  readonly languages: readonly { readonly locale: "en" | "de"; readonly overview: string; readonly spaces: string; readonly newSpace: string }[];
  readonly mutation: { readonly kind: "create-space"; readonly requestId: string; readonly name: string; readonly spaceKind: "atelier" | "studio" | "archive"; readonly visibility: "private" | "public" };
  readonly operation: { readonly kind: "rebuild-directory-projections"; readonly requestId: string };
};

type LocalBrowserRelay = { readonly url: string; readonly secret: Buffer; stop: () => Promise<void> };
type LocalAdminRelay = { readonly url: string; stop: () => Promise<void> };

type OrderedAppendBroadcastFixture = {
  readonly schema: "semio.hub.directory.ordered-append-broadcast/v1";
  readonly maximumEventsPerDecision: 2;
  readonly cases: readonly {
    readonly id: string;
    readonly persistedSequences: readonly number[];
    readonly appendSucceeds: boolean;
    readonly expectedBroadcastSequences: readonly number[];
  }[];
};

/** 📣️ Validates the neutral append/broadcast law and the exact single-writer production seam. */
export function orderedDirectoryPublicationOracle(repoRoot: string): number {
  const base = join(repoRoot, "🌎️hub/📇️directory/🧫️fixtures/📣️ordered-append-broadcast-v1");
  const fixture = JSON.parse(readFileSync(join(base, "🔣️.json"), "utf8")) as OrderedAppendBroadcastFixture;
  const schema = JSON.parse(readFileSync(join(base, "🧬️.schema.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`ordered directory publication fixture: ${JSON.stringify(validate.errors)}`);
  if (new Set(fixture.cases.map((row) => row.id)).size !== fixture.cases.length) throw new Error("ordered directory publication fixture has duplicate cases");
  for (const row of fixture.cases) {
    const expected = row.appendSucceeds ? row.persistedSequences : [];
    if (JSON.stringify(expected) !== JSON.stringify(row.expectedBroadcastSequences)) throw new Error(`ordered directory publication oracle differs for ${row.id}`);
    if (row.persistedSequences.length > fixture.maximumEventsPerDecision) throw new Error(`ordered directory publication fixture exceeds its decision bound for ${row.id}`);
  }
  const source = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🦀️.rs"), "utf8");
  const body = (text: string, name: string): string => {
    const signature = text.indexOf(`fn ${name}(`);
    if (signature < 0) return "";
    const start = text.indexOf("{", signature);
    let depth = 0;
    for (let index = start; index < text.length; index += 1) {
      if (text[index] === "{") depth += 1;
      else if (text[index] === "}" && --depth === 0) return text.slice(start + 1, index);
    }
    return "";
  };
  const exact = (text: string): boolean => {
    const append = body(text, "append_and_publish_locked");
    const publish = body(text, "publish_persisted_locked");
    const common = ["execute", "execute_create_space_with_id", "execute_artifact_authority", "redeem_invite"].map((name) => body(text, name));
    const checkpoint = body(text, "publish_reserved_artifact_checkpoint");
    return append.indexOf("self.dir.append_events(events).await?") >= 0
      && append.indexOf("self.dir.append_events(events).await?") < append.indexOf("self.publish_persisted_locked(clock, persisted)")
      && publish.includes("for event in &persisted")
      && publish.includes("self.tx.send(DirectoryStreamMessage::Event { event: event.clone() })")
      && common.every((method) => method.includes("self.append_and_publish_locked(&clock,") && !method.includes("drop(clock)"))
      && checkpoint.includes("self.publish_persisted_locked(&clock, persisted)")
      && !checkpoint.includes("drop(clock)");
  };
  if (!exact(source)) throw new Error("directory append and broadcast do not share one writer-guard lifetime");
  const hostiles = [
    source.replace("self.append_and_publish_locked(&clock, &decision.events).await?", "drop(clock); self.dir.append_events(&decision.events).await?"),
    source.replace("self.publish_persisted_locked(clock, persisted)", "drop(clock); persisted"),
    source.replace("self.publish_persisted_locked(&clock, persisted)", "drop(clock); persisted"),
  ];
  for (const hostile of hostiles) if (exact(hostile)) throw new Error("directory ordered-publication oracle accepted an unlocked append or fanout");
  return fixture.cases.length + hostiles.length;
}

const LOCAL_RELAY_MAX_BODY_BYTES = 1024 * 1024;
const LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES = 4 * 1024 * 1024;
const LOCAL_RELAY_MAX_IN_FLIGHT = 64;
const LOCAL_RELAY_DEADLINE_MS = 2_000;
const EXECUTION_TARGET_RELAY_REQUEST_MAX_BYTES = 8 * 1024;
const EXECUTION_TARGET_RELAY_MANIFEST_MAX_BYTES = 8 * 1024;
const EXECUTION_TARGET_RELAY_MAX_IN_FLIGHT = 2;
const EXECUTION_TARGET_RELAY_DEADLINE_MS = 9_000;
const BROWSER_BROKER_PROOF_DOMAIN = "semio/browser-broker-proof/v1\0";
const BROWSER_BROKER_PROOF_TTL_MS = 15_000;
const ADMIN_RELAY_BOOTSTRAP_PROOF_TTL_MS = 15_000;
const ADMIN_RELAY_SESSION_TTL_MS = 30 * 60_000;
const ADMIN_RELAY_COOKIE = "semio_admin_relay";
const ADMIN_RELAY_PROOF_DOMAIN = "semio/admin-relay-proof/v1\0";
const ADMIN_RELAY_COOKIE_DOMAIN = "semio/admin-relay-cookie/v1\0";

function browserBrokerProofDigest(proof: Uint8Array): Buffer {
  return createHash("sha256").update(BROWSER_BROKER_PROOF_DOMAIN).update(proof).digest();
}

function adminRelayDigest(domain: string, value: Uint8Array): Buffer {
  return createHash("sha256").update(domain).update(value).digest();
}

function adminRelayApiPath(method: string, url: URL): string | undefined {
  if (!url.pathname.startsWith("/admin/api/")) return undefined;
  const path = url.pathname;
  if (method === "GET" && path === "/admin/api/overview" && url.search === "") return path;
  const pageParameters = [...url.searchParams];
  const boundedPage =
    pageParameters.length >= 1 &&
    pageParameters.length <= 2 &&
    new Set(pageParameters.map(([key]) => key)).size === pageParameters.length &&
    pageParameters.every(([key, value]) => (key === "limit" && /^(?:[1-9]|[1-9]\d|100)$/u.test(value)) || (key === "cursor" && /^[0-9a-f]{84}$/u.test(value))) &&
    url.searchParams.has("limit");
  if (method === "GET" && ["/admin/api/spaces", "/admin/api/users", "/admin/api/connections", "/admin/api/events", "/admin/api/audit"].includes(path) && boundedPage) return `${path}${url.search}`;
  const documentParameters = [...url.searchParams];
  const boundedDocumentPage =
    documentParameters.length >= 1 &&
    documentParameters.length <= 3 &&
    new Set(documentParameters.map(([key]) => key)).size === documentParameters.length &&
    documentParameters.every(
      ([key, value]) =>
        (key === "limit" && /^(?:[1-9]|[1-9]\d|100)$/u.test(value)) ||
        (key === "cursor" && /^[0-9a-f]{84}$/u.test(value)) ||
        (key === "space" && Buffer.byteLength(value, "utf8") >= 1 && Buffer.byteLength(value, "utf8") <= 4096 && !/[\u0000-\u001f\u007f]/u.test(value)),
    ) &&
    url.searchParams.has("limit");
  if (method === "GET" && /^\/admin\/api\/spaces\/[^/]+$/u.test(path) && boundedPage) return `${path}${url.search}`;
  if (method === "GET" && /^\/admin\/api\/operations\/[^/]+$/u.test(path) && url.search === "") return path;
  if (method === "GET" && path === "/admin/api/documents" && boundedDocumentPage) return `${path}${url.search}`;
  if (method === "POST" && path === "/admin/api/intents" && url.search === "") return path;
  if (method === "POST" && /^\/admin\/api\/operations\/[^/]+\/cancel$/u.test(path) && url.search === "") return path;
  return undefined;
}

function adminRelayCookie(request: Request): string | undefined {
  const matches = (request.headers.get("cookie") ?? "")
    .split(";")
    .map((part) => part.trim())
    .filter((part) => part.startsWith(`${ADMIN_RELAY_COOKIE}=`));
  if (matches.length !== 1) return undefined;
  const value = matches[0]!.slice(ADMIN_RELAY_COOKIE.length + 1);
  return /^[0-9a-f]{64}$/u.test(value) ? value : undefined;
}

function startLocalAdminRelay(hubOrigin: string, envelope: Record<string, any>, bootstrapProof: Buffer, proofTtlMs = ADMIN_RELAY_BOOTSTRAP_PROOF_TTL_MS, sessionTtlMs = ADMIN_RELAY_SESSION_TTL_MS): LocalAdminRelay {
  if (!Number.isSafeInteger(proofTtlMs) || proofTtlMs <= 0 || proofTtlMs > ADMIN_RELAY_BOOTSTRAP_PROOF_TTL_MS) throw new Error("admin relay bootstrap TTL invalid");
  if (!Number.isSafeInteger(sessionTtlMs) || sessionTtlMs <= 0 || sessionTtlMs > ADMIN_RELAY_SESSION_TTL_MS) throw new Error("admin relay session TTL invalid");
  if (envelope.schema !== "semio.hub.local-credential-envelope/v1" || envelope.clientClass !== "admin-relay" || typeof envelope.capability !== "string" || !/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(envelope.capability))
    throw new Error("admin relay credential envelope binding mismatch");
  const credentialRemainingMs = Number(envelope.expiresAt) - Date.now();
  if (!Number.isSafeInteger(credentialRemainingMs) || credentialRemainingMs <= 0) throw new Error("admin relay credential envelope expired");
  const effectiveSessionTtlMs = Math.min(sessionTtlMs, credentialRemainingMs);
  let proofDigest = adminRelayDigest(ADMIN_RELAY_PROOF_DOMAIN, bootstrapProof);
  bootstrapProof.fill(0);
  let proofExpiresAtMs = Date.now() + proofTtlMs;
  let cookieDigest: Buffer | undefined;
  let cookieExpiresAtMs = 0;
  let capability = envelope.capability;
  envelope.capability = "";
  let inFlight = 0;
  let stopping = false;
  let stopPromise: Promise<void> | undefined;
  const upstreamControllers = new Set<AbortController>();
  const idleWaiters = new Set<() => void>();
  let relayOrigin = "";
  const server = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request, relayServer): Promise<Response> {
      const url = new URL(request.url);
      const origin = relayOrigin;
      const peer = relayServer.requestIP(request)?.address;
      if (stopping || !capability || url.origin !== relayOrigin || (peer !== "127.0.0.1" && peer !== "::1") || request.headers.get("host") !== new URL(relayOrigin).host) return new Response("unauthorized", { status: 401 });
      if (request.method === "POST" && url.pathname === "/__semio/admin/bootstrap" && url.search === "") {
        const supplied = request.headers.get("x-semio-admin-bootstrap");
        const referer = request.headers.get("referer");
        if (
          Date.now() > proofExpiresAtMs ||
          supplied === null ||
          !/^[0-9a-f]{64}$/u.test(supplied) ||
          request.headers.get("origin") !== origin ||
          referer === null ||
          !referer.startsWith(`${origin}/admin/`) ||
          request.headers.get("sec-fetch-site") !== "same-origin"
        )
          return new Response("unauthorized", { status: 401 });
        const suppliedBytes = Buffer.from(supplied, "hex");
        const suppliedDigest = adminRelayDigest(ADMIN_RELAY_PROOF_DOMAIN, suppliedBytes);
        suppliedBytes.fill(0);
        const admitted = timingSafeEqual(suppliedDigest, proofDigest);
        suppliedDigest.fill(0);
        if (!admitted) return new Response("unauthorized", { status: 401 });
        proofDigest.fill(0);
        proofExpiresAtMs = 0;
        const cookie = randomBytes(32);
        cookieDigest?.fill(0);
        cookieDigest = adminRelayDigest(ADMIN_RELAY_COOKIE_DOMAIN, cookie);
        cookieExpiresAtMs = Date.now() + effectiveSessionTtlMs;
        const cookieValue = cookie.toString("hex");
        cookie.fill(0);
        return new Response(null, { status: 204, headers: { "cache-control": "no-store", "set-cookie": `${ADMIN_RELAY_COOKIE}=${cookieValue}; HttpOnly; SameSite=Strict; Path=/; Max-Age=${Math.max(1, Math.floor(effectiveSessionTtlMs / 1_000))}` } });
      }
      if (request.method === "GET" && (url.pathname === "/" || url.pathname === "/admin")) return Response.redirect(`${origin}/admin/`, 302);
      if (request.method === "GET" && url.pathname.startsWith("/admin/") && !url.pathname.startsWith("/admin/api/")) {
        if (inFlight >= LOCAL_RELAY_MAX_IN_FLIGHT) return new Response("unavailable", { status: 503, headers: { "cache-control": "no-store" } });
        inFlight += 1;
        const upstreamController = new AbortController();
        const cancelUpstream = (): void => upstreamController.abort();
        request.signal.addEventListener("abort", cancelUpstream, { once: true });
        upstreamControllers.add(upstreamController);
        try {
          const upstream = await fetch(`${hubOrigin}${url.pathname}${url.search}`, { redirect: "error", signal: AbortSignal.any([upstreamController.signal, AbortSignal.timeout(LOCAL_RELAY_DEADLINE_MS)]) });
          const responseBody = await readLocalRelayResponse(upstream, LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES);
          const contentType = upstream.headers.get("content-type");
          const cacheControl = upstream.headers.get("cache-control");
          return new Response(responseBody, { status: upstream.status, headers: { ...(contentType ? { "content-type": contentType } : {}), ...(cacheControl ? { "cache-control": cacheControl } : {}) } });
        } catch {
          return new Response("unavailable", { status: 503, headers: { "cache-control": "no-store" } });
        } finally {
          request.signal.removeEventListener("abort", cancelUpstream);
          upstreamControllers.delete(upstreamController);
          inFlight -= 1;
          if (inFlight === 0) {
            for (const resolveIdle of idleWaiters) resolveIdle();
            idleWaiters.clear();
          }
        }
      }
      const upstreamPath = adminRelayApiPath(request.method, url);
      const suppliedCookie = adminRelayCookie(request);
      if (!upstreamPath || !cookieDigest || Date.now() > cookieExpiresAtMs || !suppliedCookie) return new Response("unauthorized", { status: upstreamPath ? 401 : 404 });
      const cookieBytes = Buffer.from(suppliedCookie, "hex");
      const suppliedCookieDigest = adminRelayDigest(ADMIN_RELAY_COOKIE_DOMAIN, cookieBytes);
      cookieBytes.fill(0);
      const admitted = timingSafeEqual(suppliedCookieDigest, cookieDigest);
      suppliedCookieDigest.fill(0);
      if (!admitted) return new Response("unauthorized", { status: 401 });
      if (request.method !== "GET") {
        const referer = request.headers.get("referer");
        if (request.headers.get("origin") !== origin || referer === null || !referer.startsWith(`${origin}/admin/`) || request.headers.get("sec-fetch-site") !== "same-origin") return new Response("unauthorized", { status: 401 });
      }
      if (inFlight >= LOCAL_RELAY_MAX_IN_FLIGHT) return new Response("unavailable", { status: 503, headers: { "cache-control": "no-store" } });
      inFlight += 1;
      const upstreamController = new AbortController();
      const cancelUpstream = (): void => upstreamController.abort();
      request.signal.addEventListener("abort", cancelUpstream, { once: true });
      upstreamControllers.add(upstreamController);
      try {
        const body = await readLocalRelayBody(request, upstreamPath === "/admin/api/intents" ? 8 * 1024 : LOCAL_RELAY_MAX_BODY_BYTES);
        const upstream = await fetch(`${hubOrigin}${upstreamPath}`, {
          method: request.method,
          headers: { authorization: `Bearer ${capability}`, ...(body?.byteLength ? { "content-type": "application/json" } : {}) },
          body,
          redirect: "error",
          signal: AbortSignal.any([upstreamController.signal, AbortSignal.timeout(LOCAL_RELAY_DEADLINE_MS)]),
        });
        if (upstream.status === 401) {
          capability = "";
          cookieDigest.fill(0);
          cookieDigest = undefined;
          cookieExpiresAtMs = 0;
        }
        const responseBody = await readLocalRelayResponse(upstream, 64 * 1024);
        const contentType = upstream.headers.get("content-type");
        return new Response(responseBody, {
          status: upstream.status,
          headers: { "cache-control": "no-store", ...(contentType ? { "content-type": contentType } : {}), ...(upstream.status === 401 ? { "set-cookie": `${ADMIN_RELAY_COOKIE}=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0` } : {}) },
        });
      } catch (error) {
        const requestTooLarge = error instanceof Error && error.message === "payload too large";
        return new Response(requestTooLarge ? "payload too large" : "unavailable", { status: requestTooLarge ? 413 : 503, headers: { "cache-control": "no-store" } });
      } finally {
        request.signal.removeEventListener("abort", cancelUpstream);
        upstreamControllers.delete(upstreamController);
        inFlight -= 1;
        if (inFlight === 0) {
          for (const resolveIdle of idleWaiters) resolveIdle();
          idleWaiters.clear();
        }
      }
    },
  });
  relayOrigin = `http://127.0.0.1:${server.port}`;
  return {
    url: relayOrigin,
    stop: async () => {
      if (stopPromise) return stopPromise;
      stopping = true;
      capability = "";
      proofDigest.fill(0);
      proofExpiresAtMs = 0;
      cookieDigest?.fill(0);
      cookieDigest = undefined;
      cookieExpiresAtMs = 0;
      for (const controller of upstreamControllers) controller.abort();
      stopPromise = (async () => {
        if (inFlight !== 0) await Promise.race([new Promise<void>((resolveIdle) => idleWaiters.add(resolveIdle)), Bun.sleep(2_000)]);
        await server.stop(true);
      })();
      return stopPromise;
    },
  };
}

function localRelayExecutionTargetAsset(path: string): "manifest" | "component" | "descriptor" | undefined {
  const matched = /^\/spaces\/([^/]+)\/documents\/([^/]+)\/execution-target\/(manifest|component|descriptor)$/u.exec(path);
  if (!matched) return undefined;
  try {
    for (const encoded of [matched[1]!, matched[2]!]) {
      const id = decodeURIComponent(encoded);
      if (!id || id === "." || id === ".." || encodeURIComponent(id) !== encoded || /[\/\\\u0000-\u0020\u007f%?#]/u.test(id)) return undefined;
    }
    return matched[3] as "manifest" | "component" | "descriptor";
  } catch {
    return undefined;
  }
}

function localRelayUpstreamPath(method: string, url: URL): string | undefined {
  if (!url.pathname.startsWith("/_semio/hub/")) return undefined;
  const upstream = url.pathname.slice("/_semio/hub".length);
  const noQuery = url.search === "";
  if (method === "GET" && upstream === "/auth/sessions/me" && noQuery) return upstream;
  if (method === "GET" && (upstream === "/directory/spaces" || /^\/directory\/spaces\/[^/]+$/u.test(upstream)) && noQuery) return upstream;
  if (method === "GET" && upstream === "/directory/events" && [...url.searchParams].length === 1 && /^\d+$/u.test(url.searchParams.get("since") ?? "")) return `${upstream}?since=${url.searchParams.get("since")}`;
  if (method === "POST" && (upstream === "/directory/commands" || upstream === "/directory/socket-grants") && noQuery) return upstream;
  if (method === "POST" && /^\/directory\/spaces\/[^/]+\/documents\/[^/]+\/socket-grants$/u.test(upstream) && noQuery) return upstream;
  if (method === "POST" && /^\/spaces\/[^/]+\/documents\/[^/]+\/open-plan$/u.test(upstream) && noQuery) return upstream;
  if (method === "POST" && /^\/spaces\/[^/]+\/documents\/[^/]+\/socket-grants$/u.test(upstream) && noQuery) return upstream;
  if (method === "POST" && localRelayExecutionTargetAsset(upstream) && noQuery) return upstream;
  return undefined;
}

async function readLocalRelayBody(request: Request, maximumBytes = LOCAL_RELAY_MAX_BODY_BYTES, signal?: AbortSignal): Promise<Uint8Array | undefined> {
  signal?.throwIfAborted();
  if (request.method === "GET" || request.method === "DELETE" || request.body === null) return undefined;
  const contentLength = Number(request.headers.get("content-length") ?? "0");
  if (!Number.isSafeInteger(contentLength) || contentLength < 0 || contentLength > maximumBytes) throw new Error("payload too large");
  const reader = request.body.getReader();
  const cancel = (): void => { void reader.cancel().catch(() => undefined); };
  signal?.addEventListener("abort", cancel, { once: true });
  const chunks: Uint8Array[] = [];
  let retained = 0;
  try {
    for (;;) {
      signal?.throwIfAborted();
      const { done, value } = await reader.read();
      signal?.throwIfAborted();
      if (done) break;
      retained += value.byteLength;
      if (retained > maximumBytes) {
        await reader.cancel();
        throw new Error("payload too large");
      }
      chunks.push(value);
    }
  } finally {
    signal?.removeEventListener("abort", cancel);
    reader.releaseLock();
  }
  const body = new Uint8Array(retained);
  let offset = 0;
  for (const chunk of chunks) {
    body.set(chunk, offset);
    offset += chunk.byteLength;
  }
  return body;
}

async function readLocalRelayResponse(response: Response, maximumBytes = LOCAL_RELAY_MAX_BODY_BYTES): Promise<Uint8Array> {
  const contentLength = Number(response.headers.get("content-length") ?? "0");
  if (!Number.isSafeInteger(contentLength) || contentLength < 0 || contentLength > maximumBytes) throw new Error("response too large");
  if (response.body === null) return new Uint8Array();
  const reader = response.body.getReader();
  const chunks: Uint8Array[] = [];
  let retained = 0;
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    retained += value.byteLength;
    if (retained > maximumBytes) {
      await reader.cancel();
      throw new Error("response too large");
    }
    chunks.push(value);
  }
  const body = new Uint8Array(retained);
  let offset = 0;
  for (const chunk of chunks) {
    body.set(chunk, offset);
    offset += chunk.byteLength;
  }
  return body;
}

function matchesSecret(supplied: string | null, expected: Buffer): boolean {
  if (supplied === null || !/^[0-9a-f]{64}$/u.test(supplied)) return false;
  const candidate = Buffer.from(supplied, "hex");
  const matches = candidate.length === expected.length && timingSafeEqual(candidate, expected);
  candidate.fill(0);
  return matches;
}

function startLocalBrowserRelay(hubOrigin: string, uiOrigin: string, envelope: Record<string, any>, browserProof: Buffer, proofTtlMs = BROWSER_BROKER_PROOF_TTL_MS, binding?: { readonly port: number; readonly secret: Buffer }): LocalBrowserRelay {
  if (!Number.isSafeInteger(proofTtlMs) || proofTtlMs <= 0 || proofTtlMs > BROWSER_BROKER_PROOF_TTL_MS) throw new Error("browser broker proof TTL invalid");
  if (envelope.schema !== "semio.hub.local-credential-envelope/v1" || envelope.clientClass !== "react-relay" || typeof envelope.capability !== "string" || !/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(envelope.capability))
    throw new Error("react relay credential envelope binding mismatch");
  const secret = binding?.secret ?? randomBytes(32);
  let browserProofDigest = browserBrokerProofDigest(browserProof);
  browserProof.fill(0);
  let browserProofExpiresAtMs = Date.now() + proofTtlMs;
  let capability = envelope.capability;
  envelope.capability = "";
  let inFlight = 0;
  let executionTargetsInFlight = 0;
  let stopping = false;
  let stopPromise: Promise<void> | undefined;
  const upstreamControllers = new Set<AbortController>();
  const idleWaiters = new Set<() => void>();
  const server = Bun.serve({
    hostname: "127.0.0.1",
    port: binding?.port ?? 0,
    async fetch(request, relayServer): Promise<Response> {
      const supplied = request.headers.get("x-semio-local-relay");
      const origin = request.headers.get("origin");
      const referer = request.headers.get("referer");
      const fetchSite = request.headers.get("sec-fetch-site");
      const host = request.headers.get("host");
      const peer = relayServer.requestIP(request)?.address;
      const rejection = stopping ? "stopping"
        : !capability ? "capability"
        : !matchesSecret(supplied, secret) ? "secret"
        : host !== new URL(uiOrigin).host ? "host"
        : peer !== "127.0.0.1" && peer !== "::1" ? "peer"
        : origin !== uiOrigin ? "origin"
        : referer === null || !referer.startsWith(`${uiOrigin}/`) ? "referer"
        : fetchSite !== "same-origin" ? "fetch-site"
        : "";
      if (rejection) {
        return new Response("unauthorized", { status: 401 });
      }
      const url = new URL(request.url);
      const upstreamPath = localRelayUpstreamPath(request.method, url);
      if (!upstreamPath || inFlight >= LOCAL_RELAY_MAX_IN_FLIGHT) return new Response("unavailable", { status: upstreamPath ? 503 : 404 });
      const executionTarget = localRelayExecutionTargetAsset(upstreamPath);
      if (executionTarget && executionTargetsInFlight >= EXECUTION_TARGET_RELAY_MAX_IN_FLIGHT) return new Response("unavailable", { status: 503 });
      const requestMaxBytes = executionTarget ? EXECUTION_TARGET_RELAY_REQUEST_MAX_BYTES : LOCAL_RELAY_MAX_BODY_BYTES;
      const responseMaxBytes = executionTarget === "component" ? DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES : executionTarget === "descriptor" ? DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES : executionTarget === "manifest" ? EXECUTION_TARGET_RELAY_MANIFEST_MAX_BYTES : LOCAL_RELAY_MAX_BODY_BYTES;
      const contentLength = Number(request.headers.get("content-length") ?? "0");
      if (!Number.isSafeInteger(contentLength) || contentLength < 0 || contentLength > requestMaxBytes) return new Response("payload too large", { status: 413 });
      const currentProof = request.headers.get("x-semio-browser-broker");
      const nextProofDigest = request.headers.get("x-semio-browser-broker-next");
      if (Date.now() > browserProofExpiresAtMs || currentProof === null || nextProofDigest === null || !/^[0-9a-f]{64}$/u.test(currentProof) || !/^[0-9a-f]{64}$/u.test(nextProofDigest)) {
        return new Response("unauthorized", { status: 401 });
      }
      const currentProofBytes = Buffer.from(currentProof, "hex");
      const currentDigest = browserBrokerProofDigest(currentProofBytes);
      currentProofBytes.fill(0);
      const proofMatches = timingSafeEqual(currentDigest, browserProofDigest);
      currentDigest.fill(0);
      if (!proofMatches) {
        return new Response("unauthorized", { status: 401 });
      }
      browserProofDigest.fill(0);
      browserProofDigest = Buffer.from(nextProofDigest, "hex");
      browserProofExpiresAtMs = Date.now() + proofTtlMs;
      inFlight += 1;
      if (executionTarget) executionTargetsInFlight += 1;
      const upstreamController = new AbortController();
      const cancelUpstream = (): void => upstreamController.abort();
      request.signal.addEventListener("abort", cancelUpstream, { once: true });
      upstreamControllers.add(upstreamController);
      const signal = AbortSignal.any([request.signal, upstreamController.signal, AbortSignal.timeout(executionTarget ? EXECUTION_TARGET_RELAY_DEADLINE_MS : LOCAL_RELAY_DEADLINE_MS)]);
      try {
        const body = await readLocalRelayBody(request, requestMaxBytes, signal);
        const upstream = await fetch(`${hubOrigin}${upstreamPath}`, {
          method: request.method,
          headers: { authorization: `Bearer ${capability}`, ...(body?.byteLength ? { "content-type": "application/json" } : {}) },
          body,
          redirect: "error",
          signal,
        });
        if (upstream.status === 401) capability = "";
        const responseBody = await readLocalRelayResponse(upstream, responseMaxBytes);
        const contentType = upstream.headers.get("content-type");
        return new Response(responseBody, { status: upstream.status, headers: { "x-semio-browser-broker-advanced": "1", "content-length": String(responseBody.byteLength), "cache-control": "no-store", ...(contentType ? { "content-type": contentType } : {}) } });
      } catch (error) {
        return new Response(error instanceof Error && error.message === "payload too large" ? "payload too large" : "unavailable", {
          status: error instanceof Error && error.message === "payload too large" ? 413 : 503,
          headers: { "x-semio-browser-broker-advanced": "1" },
        });
      } finally {
        request.signal.removeEventListener("abort", cancelUpstream);
        upstreamController.abort();
        upstreamControllers.delete(upstreamController);
        inFlight -= 1;
        if (executionTarget) executionTargetsInFlight -= 1;
        if (inFlight === 0) {
          for (const resolveIdle of idleWaiters) resolveIdle();
          idleWaiters.clear();
        }
      }
    },
  });
  return {
    url: `http://127.0.0.1:${server.port}`,
    secret,
    stop: () => {
      if (stopPromise) return stopPromise;
      stopping = true;
      capability = "";
      secret.fill(0);
      browserProofDigest.fill(0);
      browserProofExpiresAtMs = 0;
      for (const controller of upstreamControllers) controller.abort();
      stopPromise = (async () => {
        if (inFlight !== 0) await Promise.race([new Promise<void>((resolveIdle) => idleWaiters.add(resolveIdle)), Bun.sleep(2_000)]);
        await server.stop(true);
      })();
      return stopPromise;
    },
  };
}

class LocalFrameReader {
  private retained = Buffer.alloc(0);
  private readonly waiters: Array<{ resolve: (value: Record<string, any>) => void; reject: (error: Error) => void; timer: ReturnType<typeof setTimeout> }> = [];

  constructor(private readonly pipe: Duplex) {
    pipe.on("data", (chunk: Buffer) => {
      if (this.retained.length + chunk.length > LOCAL_BOOTSTRAP_FRAME_MAX * 2) return this.fail(new Error("local bootstrap retained input exceeded fixed bound"));
      this.retained = Buffer.concat([this.retained, chunk]);
      this.drain();
    });
    pipe.once("error", () => this.fail(new Error("local bootstrap endpoint failed")));
    pipe.once("end", () => this.fail(new Error("local bootstrap endpoint reached EOF")));
    pipe.once("close", () => this.fail(new Error("local bootstrap endpoint closed")));
  }

  read(deadlineMs = LOCAL_BOOTSTRAP_DEADLINE_MS): Promise<Record<string, any>> {
    if (!Number.isSafeInteger(deadlineMs) || deadlineMs <= 0) return Promise.reject(new Error("local bootstrap frame deadline invalid"));
    if (this.waiters.length >= 8) return Promise.reject(new Error("local bootstrap outstanding read bound exceeded"));
    return new Promise((resolveRead, rejectRead) => {
      const waiter = {
        resolve: resolveRead,
        reject: rejectRead,
        timer: setTimeout(() => {
          const index = this.waiters.indexOf(waiter);
          if (index >= 0) this.waiters.splice(index, 1);
          rejectRead(new Error("local bootstrap frame deadline exceeded"));
        }, deadlineMs),
      };
      this.waiters.push(waiter);
      this.drain();
    });
  }

  private drain(): void {
    while (this.waiters.length > 0 && this.retained.length >= 4) {
      const length = this.retained.readUInt32BE(0);
      if (length === 0 || length + 4 > LOCAL_BOOTSTRAP_FRAME_MAX) return this.fail(new Error("local bootstrap frame exceeded fixed bound"));
      if (this.retained.length < length + 4) return;
      const bytes = this.retained.subarray(4, length + 4);
      this.retained = this.retained.subarray(length + 4);
      const waiter = this.waiters.shift()!;
      clearTimeout(waiter.timer);
      try {
        waiter.resolve(JSON.parse(bytes.toString("utf8")));
      } catch {
        waiter.reject(new Error("local bootstrap frame was not JSON"));
      }
    }
  }

  private fail(error: Error): void {
    for (const waiter of this.waiters.splice(0)) {
      clearTimeout(waiter.timer);
      waiter.reject(error);
    }
    this.retained.fill(0);
    this.retained = Buffer.alloc(0);
  }
}

function hmacProof(channelKey: Buffer, unsigned: object): string {
  const canonical = Buffer.from(JSON.stringify(unsigned));
  if (canonical.length + 4 > LOCAL_BOOTSTRAP_FRAME_MAX) throw new Error("local bootstrap canonical frame exceeded fixed bound");
  const length = Buffer.alloc(4);
  length.writeUInt32BE(canonical.length);
  const proof = createHmac("sha256", channelKey).update(LOCAL_BOOTSTRAP_DOMAIN).update(length).update(canonical).digest("hex");
  canonical.fill(0);
  return proof;
}

function authenticatedFrame(channelKey: Buffer, unsigned: Record<string, unknown>): Record<string, unknown> {
  return { ...unsigned, proof: hmacProof(channelKey, unsigned) };
}

function writeLocalFrame(pipe: Duplex, value: object): Promise<void> {
  const bytes = Buffer.from(JSON.stringify(value));
  if (bytes.length === 0 || bytes.length + 4 > LOCAL_BOOTSTRAP_FRAME_MAX) throw new Error("local bootstrap frame exceeded fixed bound");
  const frame = Buffer.allocUnsafe(bytes.length + 4);
  frame.writeUInt32BE(bytes.length, 0);
  bytes.copy(frame, 4);
  bytes.fill(0);
  return new Promise((resolveWrite, rejectWrite) => {
    pipe.write(frame, (error?: Error | null) => {
      frame.fill(0);
      if (error) rejectWrite(new Error("local bootstrap write failed"));
      else resolveWrite();
    });
  });
}

function verifyAuthenticatedFrame(channelKey: Buffer, frame: Record<string, unknown>): void {
  const proof = frame.proof;
  if (typeof proof !== "string" || !/^[0-9a-f]{64}$/.test(proof)) throw new Error("local bootstrap response proof invalid");
  const unsigned = { ...frame };
  delete unsigned.proof;
  const expected = hmacProof(channelKey, unsigned);
  const left = Buffer.from(expected, "hex");
  const right = Buffer.from(proof, "hex");
  let difference = 0;
  for (let index = 0; index < left.length; index++) difference |= left[index]! ^ right[index]!;
  left.fill(0);
  right.fill(0);
  if (difference !== 0) throw new Error("local bootstrap response proof invalid");
}

async function freeLoopbackPort(): Promise<number> {
  const server = createServer();
  await new Promise<void>((resolveListen, rejectListen) => server.once("error", rejectListen).listen(0, "127.0.0.1", resolveListen));
  const address = server.address();
  if (!address || typeof address === "string") throw new Error("local bootstrap launcher could not allocate a loopback port");
  await new Promise<void>((resolveClose, rejectClose) => server.close((error) => (error ? rejectClose(error) : resolveClose())));
  return address.port;
}

function hubBinaryPath(repoRoot: string): string {
  const target = process.env.CARGO_TARGET_DIR ? resolve(process.env.CARGO_TARGET_DIR) : join(repoRoot, "target");
  return join(target, "debug", process.platform === "win32" ? "os-hub.exe" : "os-hub");
}

type LocalHubRun = {
  readonly child: ChildProcess;
  readonly pipe: Duplex;
  readonly reader: LocalFrameReader;
  readonly channelKey: Buffer;
  readonly runId: string;
  readonly port: number;
  readonly runRoot: string;
  readonly output: () => string;
  finishPromise?: Promise<void>;
};

async function startLocalHub(
  repoRoot: string,
  root: string,
  profiles: readonly LocalProfile[],
  options: { readonly port?: number; readonly dataDir?: string; readonly capture?: boolean; readonly adminSubjects?: readonly string[]; readonly isolatedSecuritySmoke?: boolean; readonly trustedCatalog?: TrustedBootstrapMaterializationV1; readonly binaryPath?: string } = {},
): Promise<LocalHubRun> {
  if (profiles.length === 0 || profiles.length > 8) throw new Error("local bootstrap profiles must contain 1..=8 entries");
  const runId = randomBytes(16).toString("hex");
  const channelKey = randomBytes(32);
  const runRoot = mkdtempSync(join(tmpdir(), "semio-hub-run-"));
  if (process.platform !== "win32") chmodSync(runRoot, 0o700);
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
  const env: Record<string, string | undefined> = {
    ...process.env,
    OS_HUB_MODE: "development",
    OS_HUB_BIND: "127.0.0.1",
    OS_HUB_PORT: String(port),
    OS_HUB_DATA: options.dataDir ?? join(runRoot, "data"),
  };
  delete env.OS_HUB_ADMIN_TOKEN;
  delete env.S_USER;
  for (const name of Object.keys(env)) if (/^S_.*TOKEN$/.test(name)) delete env[name];
  if (options.isolatedSecuritySmoke) {
    env.OS_HUB_STORAGE_BACKEND = "fs";
    env.OS_HUB_DIRECTORY_BACKEND = "sqlite";
    delete env.OS_HUB_TRUSTED_CATALOG_BUNDLE;
    delete env.OS_HUB_TRUSTED_CATALOG_PROFILE;
    delete env.OS_HUB_ADMIN_DIR;
  }
  if (options.adminSubjects?.length) env.OS_HUB_ADMIN_SUBJECTS = options.adminSubjects.join(",");
  else delete env.OS_HUB_ADMIN_SUBJECTS;
  if (options.trustedCatalog) {
    env.OS_HUB_TRUSTED_CATALOG_BUNDLE = options.trustedCatalog.bundlePath;
    env.OS_HUB_TRUSTED_CATALOG_PROFILE = options.trustedCatalog.profileId;
  }
  const outputMode: "pipe" | "inherit" = options.capture ? "pipe" : "inherit";
  const child = spawn(options.binaryPath ?? hubBinaryPath(repoRoot), [], { cwd: root, env, shell: false, stdio: ["ignore", outputMode, outputMode, "pipe"] });
  if (options.capture) {
    child.stdout?.on("data", capture);
    child.stderr?.on("data", capture);
  }
  const pipe = child.stdio[3] as Duplex;
  if (!pipe) {
    channelKey.fill(0);
    child.kill();
    await waitForChildExit(child, 2_000).catch(() => undefined);
    rmSync(runRoot, { recursive: true, force: true });
    throw new Error("local bootstrap inherited endpoint was not created");
  }
  const reader = new LocalFrameReader(pipe);
  const run: LocalHubRun = { child, pipe, reader, channelKey, runId, port, runRoot, output: () => Buffer.concat(captured).toString("utf8") };
  try {
    const initialize = {
      schema: LOCAL_BOOTSTRAP_SCHEMA,
      kind: "initialize",
      runId,
      channelKey: channelKey.toString("hex"),
      profiles,
    };
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
    if (accepted.schema !== LOCAL_BOOTSTRAP_SCHEMA || accepted.kind !== "hello-accepted" || accepted.runId !== runId || accepted.exchangeId !== helloExchange || accepted.sequence !== 1) {
      throw new Error("local bootstrap mutual hello binding mismatch");
    }
    return run;
  } catch (error) {
    let diagnostics = run.output().slice(-2_048).replaceAll(channelKey.toString("hex"), "<channel-key-redacted>");
    for (const profile of profiles) diagnostics = diagnostics.replaceAll(profile.subject, "<profile-subject-redacted>");
    const status = child.exitCode;
    await finishLocalHub(run);
    throw new Error(`local bootstrap handshake failed (child status ${status ?? "running"}): ${error instanceof Error ? error.message : "unknown"}\n${diagnostics}`);
  }
}

async function issueLocalCredential(run: LocalHubRun, profileId: string, clientClass: LocalClientClass, sequence = 2, exchangeId = randomBytes(16).toString("hex")): Promise<Record<string, any>> {
  const now = Date.now();
  const issue = authenticatedFrame(run.channelKey, {
    schema: LOCAL_BOOTSTRAP_SCHEMA,
    kind: "issue",
    runId: run.runId,
    sequence,
    exchangeId,
    issuedAt: now,
    expiresAt: now + LOCAL_BOOTSTRAP_DEADLINE_MS,
    profileId,
    deviceInstanceId: `${clientClass}-launcher`,
    clientClass,
  });
  await writeLocalFrame(run.pipe, issue);
  const envelope = await run.reader.read();
  verifyAuthenticatedFrame(run.channelKey, envelope);
  if (
    envelope.schema !== "semio.hub.local-credential-envelope/v1" ||
    envelope.runId !== run.runId ||
    envelope.exchangeId !== exchangeId ||
    envelope.profileId !== profileId ||
    envelope.clientClass !== clientClass ||
    envelope.sessionKind !== "development-local" ||
    !Number.isSafeInteger(envelope.authorizationGeneration) ||
    envelope.authorizationGeneration < 1
  ) {
    throw new Error("local credential envelope binding mismatch");
  }
  return envelope;
}

async function waitForReadiness(run: LocalHubRun, bootstrapSecuritySmoke = false): Promise<Record<string, any>> {
  const deadline = Date.now() + LOCAL_READINESS_DEADLINE_MS;
  while (Date.now() < deadline) {
    if (run.child.exitCode !== null) throw new Error("hub exited before readiness");
    try {
      const response = await fetch(`http://127.0.0.1:${run.port}/readyz`, { signal: AbortSignal.timeout(1000) });
      const body = (await response.json()) as Record<string, any>;
      if (
        body.schema !== "semio.hub.readiness/v1" ||
        body.runId !== run.runId ||
        body.mode !== "development" ||
        body.bindScope !== "loopback" ||
        body.authentication?.kind !== "local-bootstrap-pipe-v1" ||
        body.authentication?.publicSessionIssuance !== false
      ) {
        throw new Error("hub readiness binding mismatch");
      }
      const componentsReady = body.directory?.ready === true && body.storage?.ready === true && body.adminAssets?.ready === true;
      const fullyReady = response.status === 200 && body.status === "ready" && body.authentication.bootstrapReady === true && componentsReady && body.artifactAuthority?.ready === true;
      const bootstrapReadyOnly = response.status === 503 && body.status === "not-ready" && body.authentication.bootstrapReady === true && componentsReady && body.artifactAuthority?.ready === false;
      if (fullyReady || (bootstrapSecuritySmoke && bootstrapReadyOnly)) {
        return body;
      }
    } catch (error) {
      if (error instanceof Error && error.message === "hub readiness binding mismatch") throw error;
    }
    await Bun.sleep(50);
  }
  throw new Error("hub readiness deadline exceeded");
}

async function waitForUiReadiness(origin: string, child: ChildProcess): Promise<void> {
  const deadline = Date.now() + LOCAL_READINESS_DEADLINE_MS;
  while (Date.now() < deadline) {
    if (child.exitCode !== null) throw new Error("secure local UI exited before readiness");
    try {
      const response = await fetch(`${origin}/`, { redirect: "error", signal: AbortSignal.timeout(500) });
      if (response.ok && response.url === `${origin}/`) return;
    } catch {}
    await Bun.sleep(50);
  }
  throw new Error("secure local UI readiness deadline exceeded");
}

async function waitForChildExit(child: ChildProcess, deadlineMs = LOCAL_BOOTSTRAP_DEADLINE_MS + 2_000): Promise<void> {
  if (child.exitCode !== null) return;
  await Promise.race([
    new Promise<void>((resolveExit) => child.once("exit", () => resolveExit())),
    Bun.sleep(deadlineMs).then(() => {
      throw new Error("hub child exit deadline exceeded");
    }),
  ]);
}

function openExternalBrowser(url: string): void {
  const command = process.platform === "darwin" ? "open" : process.platform === "win32" ? "cmd.exe" : "xdg-open";
  const args = process.platform === "win32" ? ["/d", "/s", "/c", "start", "", url] : [url];
  const opener = spawn(command, args, { shell: false, detached: true, stdio: "ignore" });
  opener.unref();
}

async function finishLocalHub(run: LocalHubRun): Promise<void> {
  if (run.finishPromise) return run.finishPromise;
  run.finishPromise = (async () => {
    run.pipe.end();
    run.channelKey.fill(0);
    if (run.child.exitCode === null) {
      try {
        await waitForChildExit(run.child, 2_000);
      } catch {
        run.child.kill();
        await waitForChildExit(run.child, 2_000).catch(() => undefined);
      }
    }
    rmSync(run.runRoot, { recursive: true, force: true });
  })();
  return run.finishPromise;
}

const DIRECT_CHILD_BENIGN_ENV_KEY = "SEMIO_DIRECT_CHILD_BENIGN";
const DIRECT_CHILD_BENIGN_ENV_VALUE = "preserved";

function isProtectedDirectChildEnvironmentKey(key: string): boolean {
  const normalized = key.toUpperCase();
  return (
    normalized === "S_USER" ||
    normalized === "VITE_S_USER" ||
    normalized === "S_HUB_URL" ||
    normalized.includes("TOKEN") ||
    normalized.includes("SESSION") ||
    normalized.includes("CREDENTIAL") ||
    normalized.includes("BEARER") ||
    normalized.includes("CAPABILITY") ||
    normalized.includes("AUTHORIZATION") ||
    normalized.includes("COOKIE")
  );
}

function sealedDirectChildEnvironment(source: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  const environment: NodeJS.ProcessEnv = {};
  for (const [key, value] of Object.entries(source)) {
    if (!isProtectedDirectChildEnvironmentKey(key)) environment[key] = value;
  }
  environment[DIRECT_CHILD_BENIGN_ENV_KEY] = DIRECT_CHILD_BENIGN_ENV_VALUE;
  return environment;
}

function directChildEnvironment(source: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  const environment = sealedDirectChildEnvironment(source);
  environment.S_LOCAL_CREDENTIAL_FD = "3";
  return environment;
}

async function deliverCredentialEnvelopeToChild(
  executable: string,
  args: readonly string[],
  envelope: Record<string, any>,
  expectedClass: "native" | "mcp",
  hubOrigin: string,
  environmentSource: NodeJS.ProcessEnv = process.env,
): Promise<ChildProcess> {
  if (envelope.clientClass !== expectedClass) throw new Error("credential envelope client class mismatch");
  if (!/^http:\/\/127\.0\.0\.1:\d+$/u.test(hubOrigin)) throw new Error("credential hub origin mismatch");
  const child = spawn(executable, [...args], { shell: false, env: directChildEnvironment(environmentSource), stdio: expectedClass === "mcp" ? ["pipe", "pipe", "pipe", "pipe"] : ["ignore", "pipe", "pipe", "pipe"] });
  const pipe = child.stdio[3] as Duplex;
  if (!pipe) {
    child.kill();
    envelope.capability = "";
    throw new Error("one-shot credential endpoint was not created");
  }
  try {
    await writeLocalFrame(pipe, {
      schema: "semio.local.consumer-credential/v1",
      clientClass: expectedClass,
      hubOrigin,
      sessionId: envelope.sessionId,
      authorizationGeneration: envelope.authorizationGeneration,
      expiresAtMs: envelope.expiresAt,
      capability: envelope.capability,
    });
    pipe.end();
    return child;
  } catch (error) {
    child.kill();
    throw error;
  } finally {
    envelope.capability = "";
  }
}

export async function deliverNativeCredentialEnvelope(executable: string, args: readonly string[], envelope: Record<string, any>, hubOrigin: string): Promise<ChildProcess> {
  return deliverCredentialEnvelopeToChild(executable, args, envelope, "native", hubOrigin);
}

export async function deliverMcpCredentialEnvelope(executable: string, args: readonly string[], envelope: Record<string, any>, hubOrigin: string): Promise<ChildProcess> {
  return deliverCredentialEnvelopeToChild(executable, args, envelope, "mcp", hubOrigin);
}

function nativeWgpuExecutable(repoRoot: string): string {
  const targetRoot = process.env.CARGO_TARGET_DIR ? resolve(repoRoot, process.env.CARGO_TARGET_DIR) : join(repoRoot, "target");
  return join(targetRoot, "debug", process.platform === "win32" ? "semio-wgpu-native.exe" : "semio-wgpu-native");
}

function mcpExecutable(repoRoot: string): string {
  const targetRoot = process.env.CARGO_TARGET_DIR ? resolve(repoRoot, process.env.CARGO_TARGET_DIR) : join(repoRoot, "target");
  return join(targetRoot, "debug", process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp");
}

function proveMcpCredentialSourceOrder(repoRoot: string): void {
  const entrypoint = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚀️bin.rs"), "utf8");
  const workspace = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"), "utf8");
  const remote = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs"), "utf8");
  const directory = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  const runnerPaths = ["🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts"].map((path) => join(repoRoot, path)).filter(existsSync);
  if (runnerPaths.length !== 1) throw new Error("MCP runner path must resolve to exactly one physical source");
  const runner = readFileSync(runnerPaths[0]!, "utf8");
  const launch = readFileSync(join(repoRoot, ".vscode/🧩️launch.seed.jsonc"), "utf8");
  const claim = entrypoint.indexOf('claim_inherited_local_hub_credential("mcp")');
  if (claim < 0 || claim > entrypoint.indexOf("parse_args()") || !entrypoint.includes('return value == "3";')) throw new Error("MCP credential marker/claim no longer rejects non-fd3 values before argv parsing and workspace activation");
  if (entrypoint.includes('"--token" => hub.') || remote.includes("set_token(") || remote.includes("DirectoryClient::new(transport, base_url)")) throw new Error("MCP hub binding regained a raw credential carrier");
  const openHub = workspace.indexOf("pub fn open_hub(");
  const injectCredential = workspace.indexOf("set_local_hub_credential(credential)", openHub);
  const injectGrantSource = workspace.indexOf("set_hub_socket_grant_source(grant_source)", openHub);
  const returnWorkspace = workspace.indexOf("Ok(workspace)", openHub);
  if (openHub < 0 || injectCredential < openHub || injectGrantSource < injectCredential || returnWorkspace < injectGrantSource) throw new Error("MCP ArtifactHost credential/grant injection no longer precedes document access");
  if (!workspace.includes(`PROBE_PACK_SCHEMA_HASH: &str = "${MCP_PROBE_PACK_SCHEMA_HASH}"`) || !workspace.includes("authenticated_probe_document_is_known") || !workspace.includes("Some(probe_record_spec())"))
    throw new Error("MCP authenticated probe document schema binding drift");
  if (workspace.includes("probe_document_socket_surface") || workspace.includes("set_document_execution_target_lease(") || !workspace.includes("artifact_document_key(artifact_id)") || !workspace.includes("surface: Some(PROBE_SURFACE_ID.to_string())"))
    throw new Error("MCP probe document transport regained a forgeable local execution-target claim or lost its full-scope requested surface");
  if (!directory.includes('"/directory/socket-grants"') || !directory.includes('"/directory/socket/v1"') || !directory.includes("directory_socket_hello_v1()")) throw new Error("MCP directory binding no longer uses the v1 receipt/tag7 protocol");
  if (runner.includes('runCmd("cargo", ["run"') || !runner.includes("runCmd(buildMcpBinary")) throw new Error("MCP runner is not a direct binary supervisor");
  if (!launch.includes("os-hub:dev-secure-mcp")) throw new Error("MCP secure direct-child launch is not registered in the source seed");
}

const MCP_PROBE_SCHEMA = "os.agent.probe/v1";
const MCP_PROBE_PACK_SCHEMA_HASH = "9fab7cb8b71dabede955b4257fa06e2908642e0904f124b6230479f8a153041e";

async function createMcpProbeWorkspace(run: LocalHubRun, envelope: Record<string, any>): Promise<{ readonly spaceId: string; readonly documentId: string }> {
  const created = await postLiveDirectoryCommand(run, envelope.capability, liveDirectoryCommandRequestId(), { kind: "create-space", name: "MCP Socket Grant Probe", spaceKind: "studio", visibility: "private" });
  if (created.status !== 202) throw new Error(`MCP process probe could not create its workspace: ${created.status}`);
  const body = JSON.parse(created.text) as Record<string, any>;
  const event = Array.isArray(body.events) ? body.events.find((candidate: any) => candidate?.body?.kind === "space.created") : undefined;
  const spaceId = event?.body?.spaceId;
  if (typeof spaceId !== "string" || spaceId.length === 0) throw new Error("MCP process probe create-space response lacked its exact identifier");
  const documentId = "mcp-socket-grant-probe";
  const announced = await postLiveDirectoryCommand(run, envelope.capability, liveDirectoryCommandRequestId(), {
      kind: "announce-document",
      descriptor: {
        spaceId,
        documentId,
        artifactKind: "os.agent.probe",
        artifactSchema: MCP_PROBE_SCHEMA,
        owner: { pluginId: "os.mcp", packageId: "os.mcp.probe", version: "1.0.0", packageHash: "22".repeat(32) },
        packSchemaHash: MCP_PROBE_PACK_SCHEMA_HASH,
        bootstrapVersion: 1,
        bootstrapFrontier: { headSeq: 0, commitSeq: 0, epoch: 0 },
        bootstrapSnapshotHash: "33".repeat(32),
      },
  });
  if (announced.status !== 202) throw new Error(`MCP process probe could not announce its document: ${announced.status}`);
  return { spaceId, documentId };
}

async function startMcpWorkspaceChild(
  repoRoot: string,
  run: LocalHubRun,
  envelope: Record<string, any>,
  environmentSource: NodeJS.ProcessEnv = process.env,
): Promise<{ readonly child: ChildProcess; readonly spaceId: string; readonly documentId: string }> {
  const executable = mcpExecutable(repoRoot);
  if (!existsSync(executable)) throw new Error("owned MCP binary missing after build");
  const { spaceId, documentId } = await createMcpProbeWorkspace(run, envelope);
  const child = await deliverCredentialEnvelopeToChild(executable, ["stdio", "--hub", `http://127.0.0.1:${run.port}`, "--space", spaceId], structuredClone(envelope), "mcp", `http://127.0.0.1:${run.port}`, environmentSource);
  return { child, spaceId, documentId };
}

function jsonRpcResponses(chunks: readonly Buffer[]): Record<string, any>[] {
  const text = Buffer.concat(chunks).toString("utf8");
  const lines = text.split("\n");
  if (!text.endsWith("\n")) lines.pop();
  return lines.filter(Boolean).map((line) => JSON.parse(line) as Record<string, any>);
}

async function waitForJsonRpcResponse(child: ChildProcess, chunks: readonly Buffer[], id: number): Promise<Record<string, any>> {
  const deadline = Date.now() + 10_000;
  for (;;) {
    const response = jsonRpcResponses(chunks).find((candidate) => candidate.id === id);
    if (response) return response;
    if (child.exitCode !== null) throw new Error(`MCP direct child exited before JSON-RPC response ${id}: ${child.exitCode}`);
    if (Date.now() >= deadline) throw new Error(`MCP direct child JSON-RPC response ${id} deadline exceeded`);
    await Bun.sleep(20);
  }
}

async function activeMcpDocumentConnections(run: LocalHubRun, adminEnvelope: Record<string, any>, spaceId: string, documentId: string): Promise<Record<string, any>[]> {
  const rows: Record<string, any>[] = [];
  const cursors = new Set<string>();
  let cursor: string | undefined;
  for (let page = 0; page < 64; page++) {
    const url = new URL(`http://127.0.0.1:${run.port}/admin/api/connections`);
    url.searchParams.set("limit", "100");
    if (cursor) url.searchParams.set("cursor", cursor);
    const response = await fetch(url, { headers: { authorization: `Bearer ${adminEnvelope.capability}` }, signal: AbortSignal.timeout(2_000) });
    if (!response.ok) throw new Error(`MCP process probe could not inspect active connections: ${response.status}`);
    const body = await response.json();
    if (body?.source !== "recorded-sync-sessions" || !Array.isArray(body.rows) || body.rows.length > 100 || (body.nextCursor !== undefined && typeof body.nextCursor !== "string"))
      throw new Error("MCP process probe active connections response was not a bounded snapshot page");
    rows.push(...body.rows);
    if (rows.length > 4_096) throw new Error("MCP process probe active connections exceeded the fixed read ceiling");
    cursor = body.nextCursor;
    if (!cursor) return rows.filter((connection) => connection?.scope?.spaceId === spaceId && connection?.scope?.documentId === documentId);
    if (cursors.has(cursor)) throw new Error("MCP process probe active connection cursor repeated");
    cursors.add(cursor);
  }
  throw new Error("MCP process probe active connection page ceiling exceeded");
}

async function waitForMcpDocumentConnection(run: LocalHubRun, adminEnvelope: Record<string, any>, spaceId: string, documentId: string, priorSyncSessionId?: string): Promise<Record<string, any>> {
  const deadline = Date.now() + 10_000;
  while (Date.now() < deadline) {
    const connections = await activeMcpDocumentConnections(run, adminEnvelope, spaceId, documentId);
    const fresh = connections.filter((connection) => typeof connection.syncSessionId === "string" && connection.syncSessionId !== priorSyncSessionId && typeof connection.authenticatedUserId === "string" && !("actor" in connection));
    if (fresh.length === 1 && (priorSyncSessionId === undefined || connections.every((connection) => connection.syncSessionId !== priorSyncSessionId))) return fresh[0]!;
    await Bun.sleep(25);
  }
  throw new Error("MCP document SocketGrant/Session connection deadline exceeded");
}

function mcpDocumentGrantSelectorDigests(diagnostics: string, spaceId: string, documentId: string): string[] {
  const path = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`;
  const prefix = `[semio-directory-client] socket-grant-selector-digest ${path} `;
  return diagnostics
    .split("\n")
    .filter((line) => line.startsWith(prefix))
    .map((line) => line.slice(prefix.length))
    .filter((digest) => /^[0-9a-f]{64}$/u.test(digest));
}

async function waitForMcpDocumentGrantSelector(child: ChildProcess, chunks: readonly Buffer[], spaceId: string, documentId: string, count: number): Promise<string[]> {
  const deadline = Date.now() + 10_000;
  for (;;) {
    const digests = mcpDocumentGrantSelectorDigests(Buffer.concat(chunks).toString("utf8"), spaceId, documentId);
    if (digests.length >= count) return digests;
    if (child.exitCode !== null) throw new Error(`MCP direct child exited before document SocketGrant receipt ${count}: ${child.exitCode}`);
    if (Date.now() >= deadline) throw new Error(`MCP document SocketGrant receipt ${count} deadline exceeded`);
    await Bun.sleep(20);
  }
}

async function waitForMcpDocumentHead(run: LocalHubRun, adminEnvelope: Record<string, any>, spaceId: string, documentId: string, minimumHeadSeq: number): Promise<Record<string, any>> {
  const deadline = Date.now() + 10_000;
  while (Date.now() < deadline) {
    const response = await fetch(`http://127.0.0.1:${run.port}/admin/api/documents?limit=100&space=${encodeURIComponent(spaceId)}`, { headers: { authorization: `Bearer ${adminEnvelope.capability}` }, signal: AbortSignal.timeout(2_000) });
    if (!response.ok) throw new Error(`MCP process probe could not inspect document state: ${response.status}`);
    const body = await response.json();
    if (!Array.isArray(body?.rows) || body.rows.length > 100) throw new Error("MCP process probe document response was not a bounded page");
    const document = body.rows.find((candidate: Record<string, any>) => candidate?.descriptor?.documentId === documentId);
    if (Number.isSafeInteger(document?.headSeq) && document.headSeq >= minimumHeadSeq) return document;
    await Bun.sleep(25);
  }
  throw new Error(`MCP document command was not durably accepted through head sequence ${minimumHeadSeq}`);
}

async function proveMcpWorkspaceProcess(repoRoot: string, run: LocalHubRun, envelope: Record<string, any>, adminEnvelope: Record<string, any>): Promise<void> {
  const protectedValues = ["poison-user", "poison-session", "poison-token", "poison-origin", "poison-auth", "poison-cookie"];
  const poisonedEnvironment = {
    ...process.env,
    [DIRECT_CHILD_BENIGN_ENV_KEY]: DIRECT_CHILD_BENIGN_ENV_VALUE,
    SEMIO_DIRECT_CHILD_PROBE: "1",
    S_USER: protectedValues[0],
    S_SESSION: protectedValues[1],
    NPM_TOKEN: protectedValues[2],
    S_HUB_URL: protectedValues[3],
    AUTHORIZATION: protectedValues[4],
    COOKIE: protectedValues[5],
  };
  const { child, spaceId, documentId } = await startMcpWorkspaceChild(repoRoot, run, envelope, poisonedEnvironment);
  const stdout: Buffer[] = [];
  const stderr: Buffer[] = [];
  let stdoutBytes = 0;
  let stderrBytes = 0;
  child.stdout?.on("data", (chunk: Buffer) => {
    stdoutBytes += chunk.length;
    if (stdoutBytes <= 32_768) stdout.push(Buffer.from(chunk));
  });
  child.stderr?.on("data", (chunk: Buffer) => {
    stderrBytes += chunk.length;
    if (stderrBytes <= 16_384) stderr.push(Buffer.from(chunk));
  });
  try {
    const deadline = Date.now() + 10_000;
    while (!Buffer.concat(stderr).toString("utf8").includes("real per-capability ArtifactChannel routing bound")) {
      if (child.exitCode !== null) throw new Error(`MCP direct child exited before its authenticated workspace bound: ${child.exitCode}`);
      if (Date.now() >= deadline) throw new Error("MCP direct child did not bind its authenticated workspace before deadline");
      await Bun.sleep(20);
    }
    child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-11-25", capabilities: {}, clientInfo: { name: "semio-direct-child-oracle", version: "1" } } })}\n`);
    child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
    child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", id: 2, method: "tools/call", params: { name: "artifact_open", arguments: { artifactId: documentId } } })}\n`);
    const opened = await waitForJsonRpcResponse(child, stdout, 2);
    if (opened.result?.isError !== false || opened.result?.structuredContent?.artifactId !== documentId) throw new Error("MCP artifact_open did not open the authenticated document transport");
    const firstConnection = await waitForMcpDocumentConnection(run, adminEnvelope, spaceId, documentId);
    const firstSelectors = await waitForMcpDocumentGrantSelector(child, stderr, spaceId, documentId, 1);
    const persisted = await waitForMcpDocumentHead(run, adminEnvelope, spaceId, documentId, 1);
    const firstSessionId = firstConnection.syncSessionId as string;
    const firstAuthenticatedUserId = firstConnection.authenticatedUserId as string;
    const closed = await fetch(`http://127.0.0.1:${run.port}/admin/api/intents`, {
      method: "POST",
      headers: { authorization: `Bearer ${adminEnvelope.capability}`, "content-type": "application/json" },
      body: JSON.stringify({
        kind: "kick-connection",
        requestId: `mcp-reconnect-${randomBytes(12).toString("hex")}`,
        syncSessionId: firstSessionId,
        reasonCode: "mcp-process-reconnect-law",
      }),
      signal: AbortSignal.timeout(2_000),
    });
    const closedReceipt = closed.ok ? await closed.json() : undefined;
    if (closed.status !== 200 || closedReceipt?.state !== "succeeded" || closedReceipt?.outcome?.code !== "connection-kick-signalled" || closedReceipt?.outcome?.kickSignalled !== 1) {
      throw new Error(`MCP process probe could not force document reconnect through admin intent: ${closed.status}`);
    }
    const reconnected = await waitForMcpDocumentConnection(run, adminEnvelope, spaceId, documentId, firstSessionId);
    const reconnectedSelectors = await waitForMcpDocumentGrantSelector(child, stderr, spaceId, documentId, 2);
    if (reconnected.authenticatedUserId !== firstAuthenticatedUserId) throw new Error("MCP document reconnect changed its authenticated subject");
    if (reconnected.syncSessionId === firstSessionId) throw new Error("MCP document reconnect reused its sync-session identity");
    if (reconnectedSelectors[1] === firstSelectors[0]) throw new Error("MCP document reconnect reused its one-use SocketGrant selector");
    child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", id: 3, method: "tools/call", params: { name: "artifact_snapshot", arguments: { artifactId: documentId } } })}\n`);
    const snapshotted = await waitForJsonRpcResponse(child, stdout, 3);
    if (snapshotted.result?.isError !== false || snapshotted.result?.structuredContent?.artifactId !== documentId) throw new Error("MCP post-reconnect artifact state-machine operation failed");
    if (snapshotted.result?.structuredContent?.headSeq !== undefined && snapshotted.result.structuredContent.headSeq < persisted.headSeq) throw new Error("MCP post-reconnect snapshot regressed behind the persisted document frontier");
    child.stdin?.end();
    await waitForChildExit(child, 5_000);
    const output = Buffer.concat(stdout);
    const diagnostics = Buffer.concat(stderr).toString("utf8");
    const outputText = output.toString("utf8");
    let responses: Record<string, any>[] = [];
    try {
      responses = jsonRpcResponses(stdout);
    } catch {}
    const initialize = responses.find((response) => response.id === 1);
    if (child.exitCode !== 0 || stdoutBytes !== output.length || stdoutBytes > 32_768 || stderrBytes > 16_384 || responses.length !== 3 || initialize?.jsonrpc !== "2.0" || initialize?.result?.protocolVersion !== "2025-11-25") {
      throw new Error(`MCP direct child byte-clean JSON-RPC law failed: exit=${child.exitCode} stdoutBytes=${stdoutBytes} stderrBytes=${stderrBytes}`);
    }
    if (output.includes(Buffer.from(envelope.capability)) || diagnostics.includes(envelope.capability) || protectedValues.some((value) => outputText.includes(value) || diagnostics.includes(value)))
      throw new Error("MCP direct child leaked protected parent state");
  } finally {
    stdout.forEach((bytes) => bytes.fill(0));
    stderr.forEach((bytes) => bytes.fill(0));
    if (child.exitCode === null) child.kill();
  }
}

async function proveNativeCredentialEnvelopeDelivery(repoRoot: string, envelope: Record<string, any>, hubOrigin: string): Promise<void> {
  const executable = nativeWgpuExecutable(repoRoot);
  if (!existsSync(executable)) throw new Error("owned WGPU native binary missing after native-build");
  const protectedValues = ["poison-user", "poison-session", "poison-token", "poison-origin", "poison-auth", "poison-cookie"];
  const poisonedEnvironment = {
    ...process.env,
    [DIRECT_CHILD_BENIGN_ENV_KEY]: DIRECT_CHILD_BENIGN_ENV_VALUE,
    S_USER: protectedValues[0],
    S_SESSION: protectedValues[1],
    NPM_TOKEN: protectedValues[2],
    S_HUB_URL: protectedValues[3],
    AUTHORIZATION: protectedValues[4],
    COOKIE: protectedValues[5],
  };
  const child = await deliverCredentialEnvelopeToChild(executable, ["--credential-probe"], structuredClone(envelope), "native", hubOrigin, poisonedEnvironment);
  let output = "";
  let diagnostics = "";
  child.stdout?.on("data", (chunk: Buffer) => {
    if (output.length < 256) output += chunk.toString("utf8");
  });
  child.stderr?.on("data", (chunk: Buffer) => {
    if (diagnostics.length < 4_096) diagnostics += chunk.toString("utf8");
  });
  await waitForChildExit(child, 5_000);
  if (child.exitCode !== 0 || output !== "native-credential-probe-ok\n" || output.includes(envelope.capability) || diagnostics.includes(envelope.capability) || protectedValues.some((value) => output.includes(value) || diagnostics.includes(value)))
    throw new Error("actual WGPU credential claim/descendant environment seal failed");
  await proveNonFd3CredentialMarkerRejection(executable, "native");
}

async function proveNonFd3CredentialMarkerRejection(executable: string, client: "native" | "mcp"): Promise<void> {
  const poison = `session.v1.${"c".repeat(32)}.${"d".repeat(64)}`;
  const environment = sealedDirectChildEnvironment(process.env);
  environment.S_LOCAL_CREDENTIAL_FD = poison;
  const child = spawn(executable, ["--assert-no-local-credential-state"], { shell: false, env: environment, stdio: ["ignore", "pipe", "pipe"] });
  let output = "";
  let diagnostics = "";
  child.stdout?.on("data", (chunk: Buffer) => {
    if (output.length < 1_024) output += chunk.toString("utf8");
  });
  child.stderr?.on("data", (chunk: Buffer) => {
    if (diagnostics.length < 4_096) diagnostics += chunk.toString("utf8");
  });
  await waitForChildExit(child, 5_000);
  if (child.exitCode === 0 || output.includes(poison) || diagnostics.includes(poison)) throw new Error(`${client} entrypoint admitted or leaked a non-fd3 credential marker`);
}

function proveNativeCredentialSourceOrder(repoRoot: string): void {
  const entrypoint = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs"), "utf8");
  const credential = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  const runner = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts"), "utf8");
  const launch = readFileSync(join(repoRoot, ".vscode/🧩️launch.seed.jsonc"), "utf8");
  const claim = entrypoint.indexOf('claim_inherited_local_hub_credential("native")');
  if (claim < 0 || claim > entrypoint.indexOf('arg_value("--plugin")') || claim > entrypoint.indexOf("run_native(") || !entrypoint.includes('return value == "3";'))
    throw new Error("WGPU credential marker/claim no longer rejects non-fd3 values before plugin/renderer activation");
  if (!credential.includes("FD_CLOEXEC") || !credential.includes("_close(3)") || !entrypoint.includes("--assert-no-local-credential-state") || !entrypoint.includes("protected_credential_environment_is_absent"))
    throw new Error("WGPU inherited descriptor/environment seal law drift");
  if (runner.includes('const cargoArgs = ["run"]') || !runner.includes("runCmdStatus(nativeBinaryPath(ship)")) throw new Error("WGPU native runner is not a direct binary supervisor");
  if (!launch.includes("os-hub:dev-secure-native")) throw new Error("WGPU secure direct-child launch is not registered in the source seed");
}

async function proveNativeSocketGrantActor(repoRoot: string): Promise<void> {
  const actor = `hub.v1.${"a".repeat(64)}`;
  const wrongActor = `hub.v1.${"b".repeat(64)}`;
  const capability = `session.v1.${"c".repeat(32)}.${"d".repeat(64)}`;
  const issued = new Map<string, boolean>();
  const planReceipts = new Map<string, boolean>();
  const probePackSchemaHash = "9fab7cb8b71dabede955b4257fa06e2908642e0904f124b6230479f8a153041e";
  let planCount = 0;
  let exchangeCount = 0;
  let grantCount = 0;
  let socketCount = 0;
  let helloCount = 0;
  let mutationCount = 0;
  let preSessionCommands = 0;
  const frontier = (ordinal: number): WireFrontierSummary => ({
    document_id: "probe-document",
    head_edit_ordinal: ordinal,
    head_edit_id: ordinal === 0 ? "" : `probe-${ordinal}`,
    last_commit_seq: ordinal,
    chain_hash: new Array(32).fill(ordinal),
  });
  const server = Bun.serve<{ connection: number }>({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request, control): Promise<Response | undefined> {
      const url = new URL(request.url);
      if (request.method === "POST" && url.pathname === "/spaces/probe-space/documents/probe-document/open-plan") {
        if (request.headers.get("authorization") !== `Bearer ${capability}` || request.headers.get("content-length") === "0") return new Response("", { status: 401 });
        const intent = await request.json().catch(() => undefined) as Record<string, any> | undefined;
        if (
          intent?.schema !== "semio.hub.document-open-intent/v1" ||
          intent?.version !== 1 ||
          intent?.scope?.spaceId !== "probe-space" ||
          intent?.scope?.documentId !== "probe-document" ||
          intent?.requestedSurfaceId !== "native.socket-grant.probe.editor" ||
          typeof intent?.clientInstanceId !== "string" ||
          !/^native-document-[0-9a-f]{16}$/u.test(intent.clientInstanceId) ||
          Object.keys(intent).some((key) => !["schema", "version", "scope", "requestedSurfaceId", "clientInstanceId"].includes(key))
        ) return new Response("", { status: 400 });
        planCount += 1;
        const planReceipt = `open.v1.${Buffer.alloc(32, planCount).toString("base64url")}`;
        planReceipts.set(planReceipt, false);
        const descriptorDigestV1 = "11".repeat(32);
        return Response.json({
          schema: "semio.hub.document-open-plan/v1",
          version: 1,
          receipt: planReceipt,
          expiresAtUnixMs: Date.now() + 20_000,
          scope: { spaceId: "probe-space", documentId: "probe-document" },
          descriptorDigestV1,
          catalog: { generationId: "22".repeat(32) },
          package: {
            pluginId: "native.socket-grant.probe",
            packageId: "native.socket-grant.probe.codec",
            version: "1.0.0",
            componentSha256: "33".repeat(32),
            componentBlake3: "44".repeat(32),
            descriptorByteSha256: "55".repeat(32),
          },
          artifact: { kind: "native.socket-grant.probe", schema: "native.socket-grant.probe/v1", packSchemaHash: probePackSchemaHash },
          parentDialect: { artifactKind: "native.socket-grant.probe", standard: "1", subset: "*" },
          surface: {
            surfaceId: "native.socket-grant.probe.editor",
            appId: "native.socket-grant.probe.app",
            windowKindId: "native.socket-grant.probe.window",
            role: "editor",
            rendererTarget: "wgpu",
          },
          grant: { read: true, write: true, observe: true },
          revalidation: { directoryRevision: planCount, membershipGeneration: 1, sessionGeneration: 1 },
        });
      }
      if (request.method === "POST" && url.pathname === "/spaces/probe-space/documents/probe-document/socket-grants") {
        if (request.headers.get("authorization") !== `Bearer ${capability}`) return new Response("", { status: 401 });
        const exchange = await request.json().catch(() => undefined) as Record<string, any> | undefined;
        if (
          exchange?.schema !== "semio.hub.document-plan-socket-grant-intent/v1" ||
          exchange?.version !== 1 ||
          typeof exchange?.planReceipt !== "string" ||
          planReceipts.get(exchange.planReceipt) !== false ||
          Object.keys(exchange).some((key) => !["schema", "version", "planReceipt"].includes(key))
        ) return new Response("", { status: 401 });
        planReceipts.set(exchange.planReceipt, true);
        exchangeCount += 1;
        grantCount += 1;
        const selector = grantCount.toString(16).padStart(32, "0");
        const grant = `socket.v1.${selector}.${String(grantCount).padStart(64, "0")}`;
        issued.set(grant, false);
        return Response.json({ schema: "semio.hub.socket-grant/v1", protocol: "semio.socket.v1", grant, actorId: actor, expiresAtMs: Date.now() + 10_000 });
      }
      if (request.method === "GET" && url.pathname === "/spaces/probe-space/documents/probe-document/socket/v1") {
        const protocols = (request.headers.get("sec-websocket-protocol") ?? "").split(",").map((value) => value.trim());
        const grant = protocols[1] ?? "";
        if (protocols.length !== 2 || protocols[0] !== "semio.socket.v1" || issued.get(grant) !== false) return new Response("", { status: 401 });
        issued.set(grant, true);
        socketCount += 1;
        return control.upgrade(request, { data: { connection: socketCount }, headers: { "Sec-WebSocket-Protocol": "semio.socket.v1" } }) ? undefined : new Response("", { status: 500 });
      }
      return new Response("", { status: 404 });
    },
    websocket: {
      message(socket, message): void {
        const bytes = typeof message === "string" ? new TextEncoder().encode(message) : new Uint8Array(message);
        const decoded = decodeClientFrame(bytes);
        if (typeof decoded.frame !== "string" && "SocketHelloV1" in decoded.frame) {
          helloCount += 1;
          socket.send(encodeServerFrame({ Welcome: { session_id: `probe-session-${socket.data.connection}`, resume_token: `probe-resume-${socket.data.connection}`, server_frontier: frontier(mutationCount), bootstrap: "None" } }, "command"));
          socket.send(encodeServerFrame({ Session: { actor: socket.data.connection === 1 ? wrongActor : actor, color: socket.data.connection } }, "command"));
          return;
        }
        if (typeof decoded.frame !== "string" && "Commands" in decoded.frame) {
          if (socket.data.connection === 1) {
            preSessionCommands += 1;
            socket.close(1008, "command before matching Session");
            return;
          }
          if (decoded.frame.Commands.envelopes.length !== 1 || decoded.frame.Commands.envelopes[0]?.actor !== actor) {
            socket.close(1008, "actor binding");
            return;
          }
          mutationCount += 1;
          socket.send(encodeServerFrame({ Ack: { batch_id: decoded.frame.Commands.batch_id, stages: ["Received", "Persisted", { Applied: { outcome: "Accepted" } }], frontier: frontier(mutationCount) } }, "command"));
          if (socket.data.connection === 2) setTimeout(() => socket.close(1012, "reconnect"), 25);
        }
      },
    },
  });
  const envelope = { clientClass: "native", sessionId: "probe-session", authorizationGeneration: 1, expiresAt: Date.now() + 15_000, capability };
  let output = "";
  let errorOutput = "";
  try {
    const child = await deliverNativeCredentialEnvelope(nativeWgpuExecutable(repoRoot), ["--socket-grant-probe"], envelope, `http://127.0.0.1:${server.port}`);
    child.stdout?.on("data", (chunk: Buffer) => {
      if (output.length < 512) output += chunk.toString("utf8");
    });
    child.stderr?.on("data", (chunk: Buffer) => {
      if (errorOutput.length < 512) errorOutput += chunk.toString("utf8");
    });
    await waitForChildExit(child, 15_000);
    if (
      child.exitCode !== 0 ||
      output !== "native-socket-grant-probe-ok\n" ||
      planCount !== 3 ||
      exchangeCount !== 3 ||
      grantCount !== 3 ||
      socketCount !== 3 ||
      helloCount !== 3 ||
      mutationCount !== 2 ||
      preSessionCommands !== 0 ||
      [...planReceipts.values()].some((used) => !used) ||
      [...issued.values()].some((used) => !used)
    ) {
      throw new Error(`native socket actor law failed: exit=${child.exitCode} plans=${planCount} exchanges=${exchangeCount} grants=${grantCount} sockets=${socketCount} hellos=${helloCount} mutations=${mutationCount} preSessionCommands=${preSessionCommands} stderr=${errorOutput}`);
    }
    if (output.includes(capability) || errorOutput.includes(capability) || [...planReceipts.keys(), ...issued.keys()].some((secret) => output.includes(secret) || errorOutput.includes(secret)))
      throw new Error("native socket actor law leaked protected admission material");
  } finally {
    envelope.capability = "";
    server.stop(true);
  }
}

async function proveCredentialEnvelopeDelivery(envelope: Record<string, any>, clientClass: "native" | "mcp", hubOrigin: string): Promise<void> {
  const consumer = String.raw`
const fs = require("node:fs");
const expectedClass = process.argv.at(-1);
const pipe = fs.createReadStream(null, { fd: 3, autoClose: true });
const chunks = [];
let retained = 0;
const envelope = new Promise((resolve, reject) => {
  pipe.on("data", chunk => { retained += chunk.length; if (retained > 16384) reject(new Error("oversize")); else chunks.push(chunk); });
  pipe.on("error", reject);
  pipe.on("end", () => {
    const framed = Buffer.concat(chunks);
    if (framed.length < 5 || framed.readUInt32BE(0) !== framed.length - 4) return reject(new Error("framing"));
    const value = JSON.parse(framed.subarray(4).toString("utf8"));
    if (value.schema !== "semio.local.consumer-credential/v1" || value.clientClass !== expectedClass || !/^http:\/\/127\.0\.0\.1:\d+$/.test(value.hubOrigin) || !Number.isSafeInteger(value.authorizationGeneration) || value.authorizationGeneration < 1 || !/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/.test(value.capability)) return reject(new Error("binding"));
    framed.fill(0);
    resolve(undefined);
  });
});
const protocol = expectedClass === "mcp" ? new Promise((resolve, reject) => {
  let text = "";
  process.stdin.setEncoding("utf8");
  process.stdin.on("data", chunk => { text += chunk; if (text.length > 256) reject(new Error("stdio oversize")); });
  process.stdin.on("end", () => text === "{\"jsonrpc\":\"2.0\",\"id\":1}\n" ? resolve(undefined) : reject(new Error("stdio changed")));
}) : Promise.resolve();
Promise.all([envelope, protocol]).then(() => process.stdout.write("credential-delivery-ok")).catch(() => process.exit(2));`;
  const delivery = clientClass === "native" ? deliverNativeCredentialEnvelope : deliverMcpCredentialEnvelope;
  const child = await delivery(process.execPath, ["-e", consumer, clientClass], structuredClone(envelope), hubOrigin);
  let output = "";
  child.stdout?.on("data", (chunk: Buffer) => {
    if (output.length < 256) output += chunk.toString("utf8");
  });
  if (clientClass === "mcp") child.stdin?.end('{"jsonrpc":"2.0","id":1}\n');
  await waitForChildExit(child, 5_000);
  if (child.exitCode !== 0 || output !== "credential-delivery-ok") throw new Error(`${clientClass} credential delivery failed`);
}

type RejectionCase = "wrong-hmac" | "wrong-class" | "wrong-profile" | "expired" | "cross-run" | "timeout" | "eof";

async function proveRejectedBootstrap(repoRoot: string, root: string, rejection: RejectionCase): Promise<void> {
  const profiles: readonly LocalProfile[] = [{ profileId: "developer", subject: "negative-local-developer", displayName: "Negative Developer", allowedClientClasses: ["native", "mcp"] }];
  const run = await startLocalHub(repoRoot, root, profiles, { capture: true, isolatedSecuritySmoke: true });
  const keyEvidence = run.channelKey.toString("hex");
  try {
    await waitForReadiness(run, true);
    if (rejection === "eof") {
      run.pipe.end();
    } else if (rejection !== "timeout") {
      const now = Date.now();
      const unsigned: Record<string, unknown> = {
        schema: LOCAL_BOOTSTRAP_SCHEMA,
        kind: "issue",
        runId: rejection === "cross-run" ? randomBytes(16).toString("hex") : run.runId,
        sequence: 2,
        exchangeId: randomBytes(16).toString("hex"),
        issuedAt: rejection === "expired" ? now - 2_000 : now,
        expiresAt: rejection === "expired" ? now - 1_000 : now + LOCAL_BOOTSTRAP_DEADLINE_MS,
        profileId: rejection === "wrong-profile" ? "unknown" : "developer",
        deviceInstanceId: "negative-launcher",
        clientClass: rejection === "wrong-class" ? "admin-relay" : "native",
      };
      const frame = authenticatedFrame(run.channelKey, unsigned);
      if (rejection === "wrong-hmac") frame.proof = `${frame.proof === "0".repeat(64) ? "1" : "0"}${String(frame.proof).slice(1)}`;
      await writeLocalFrame(run.pipe, frame);
    }
    await waitForChildExit(run.child).catch((error: unknown) => {
      throw new Error(`${rejection} rejection did not stop the hub: ${error instanceof Error ? error.message : "unknown"}`);
    });
    if (run.child.exitCode === 0) throw new Error(`${rejection} unexpectedly kept the hub usable`);
    const output = run.output();
    if (output.includes(keyEvidence) || output.includes(profiles[0]!.subject)) throw new Error(`${rejection} leaked private bootstrap material`);
  } finally {
    await finishLocalHub(run);
  }
}

async function runSecureLocalSmoke(repoRoot: string, root: string): Promise<void> {
  const profiles: readonly LocalProfile[] = [
    { profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] },
    { profileId: "administrator", subject: "local-administrator-01", displayName: "Local Administrator", allowedClientClasses: ["native"] },
  ];
  const run = await startLocalHub(repoRoot, root, profiles, { capture: true, adminSubjects: ["semio.local.bootstrap/v1:local-administrator-01"], isolatedSecuritySmoke: true });
  const capabilities: string[] = [];
  const keyEvidence = run.channelKey.toString("hex");
  try {
    const nativeEnvelope = await issueLocalCredential(run, "developer", "native", 2);
    const mcpEnvelope = await issueLocalCredential(run, "developer", "mcp", 3);
    const adminEnvelope = await issueLocalCredential(run, "administrator", "native", 4);
    capabilities.push(nativeEnvelope.capability, mcpEnvelope.capability, adminEnvelope.capability);
    let crossClassRejected = false;
    try {
      await deliverMcpCredentialEnvelope(process.execPath, [], structuredClone(nativeEnvelope), `http://127.0.0.1:${run.port}`);
    } catch {
      crossClassRejected = true;
    }
    if (!crossClassRejected) throw new Error("native envelope crossed into MCP delivery");
    await proveNativeCredentialEnvelopeDelivery(repoRoot, nativeEnvelope, `http://127.0.0.1:${run.port}`);
    await proveCredentialEnvelopeDelivery(mcpEnvelope, "mcp", `http://127.0.0.1:${run.port}`);
    await proveNonFd3CredentialMarkerRejection(mcpExecutable(repoRoot), "mcp");
    const readiness = await waitForReadiness(run, true);
    if (readiness.status !== "not-ready" || readiness.authentication.bootstrapReady !== true || readiness.artifactAuthority?.ready !== false) throw new Error("security smoke did not observe truthful partial readiness");
    const readinessJson = JSON.stringify(readiness);
    if (capabilities.some((capability) => readinessJson.includes(capability)) || profiles.some((profile) => readinessJson.includes(profile.subject)) || readinessJson.includes("sessionKind") || readinessJson.includes("authorizationGeneration"))
      throw new Error("readiness leaked private bootstrap material");
    await proveMcpWorkspaceProcess(repoRoot, run, mcpEnvelope, adminEnvelope);
    for (let index = 0; index < capabilities.length; index++) {
      const me = await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, { headers: { authorization: `Bearer ${capabilities[index]}` } });
      if (!me.ok) throw new Error("issued local session did not validate");
      const session = (await me.json()) as Record<string, unknown>;
      const envelope = [nativeEnvelope, mcpEnvelope, adminEnvelope][index]!;
      if (session.sessionKind !== "development-local" || session.authorizationGeneration !== envelope.authorizationGeneration) throw new Error("issued local session metadata did not match its signed envelope");
    }
    const developerAdmin = await fetch(`http://127.0.0.1:${run.port}/admin/api/overview`, { headers: { authorization: `Bearer ${capabilities[0]}` } });
    const administratorAdmin = await fetch(`http://127.0.0.1:${run.port}/admin/api/overview`, { headers: { authorization: `Bearer ${capabilities[2]}` } });
    if (developerAdmin.status !== 401 || !administratorAdmin.ok) throw new Error("local administrator subject policy was not enforced");
    if ((await fetch(`http://127.0.0.1:${run.port}/auth/sessions`, { method: "POST" })).status !== 404) throw new Error("public session mint route exists");
    for (const capability of capabilities) {
      const headers = { authorization: `Bearer ${capability}` };
      if ((await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, { method: "DELETE", headers })).status !== 204) throw new Error("local session revoke failed");
      if ((await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, { headers })).status !== 401) throw new Error("revoked local session remained usable");
    }
    const replayNow = Date.now();
    await writeLocalFrame(
      run.pipe,
      authenticatedFrame(run.channelKey, {
        schema: LOCAL_BOOTSTRAP_SCHEMA,
        kind: "issue",
        runId: run.runId,
        sequence: 5,
        exchangeId: nativeEnvelope.exchangeId,
        issuedAt: replayNow,
        expiresAt: replayNow + LOCAL_BOOTSTRAP_DEADLINE_MS,
        profileId: "developer",
        deviceInstanceId: "native-launcher",
        clientClass: "native",
      }),
    );
    await waitForChildExit(run.child);
    const output = run.output();
    if (capabilities.some((capability) => output.includes(capability)) || output.includes(keyEvidence) || profiles.some((profile) => output.includes(profile.subject))) throw new Error("hub logs leaked private bootstrap material");
  } finally {
    capabilities.fill("");
    await finishLocalHub(run);
  }
  for (const rejection of ["wrong-hmac", "wrong-class", "wrong-profile", "expired", "cross-run", "timeout", "eof"] as const) {
    await proveRejectedBootstrap(repoRoot, root, rejection).catch((error: unknown) => {
      throw new Error(`${rejection} rejection failed: ${error instanceof Error ? error.message : "unknown"}`);
    });
  }
  console.log("secure-local-smoke: truthful partial readiness, native/MCP delivery, admin isolation, issue/validate/revoke, replay/HMAC/class/profile/run/expiry/timeout/EOF rejection, redaction, and absent public mint passed");
}

type BrowserBrokerOracleUpstream = { effects: number; status: number; hold: boolean };

function browserBrokerOracleEnvelope(): Record<string, unknown> {
  return { schema: "semio.hub.local-credential-envelope/v1", clientClass: "react-relay", capability: `session.v1.${"1".repeat(32)}.${"2".repeat(64)}` };
}

async function browserBrokerOracleRequest(relay: LocalBrowserRelay, uiOrigin: string, proof?: string, nextProof?: Buffer, path = "/_semio/hub/auth/sessions/me", signal?: AbortSignal): Promise<Response> {
  const headers: Record<string, string> = {
    host: new URL(uiOrigin).host,
    origin: uiOrigin,
    referer: `${uiOrigin}/`,
    "sec-fetch-site": "same-origin",
    "x-semio-local-relay": relay.secret.toString("hex"),
  };
  if (proof) headers["x-semio-browser-broker"] = proof;
  if (nextProof) headers["x-semio-browser-broker-next"] = browserBrokerProofDigest(nextProof).toString("hex");
  return fetch(`${relay.url}${path}`, { headers, redirect: "error", signal });
}

async function waitForBrowserBrokerEffect(upstream: BrowserBrokerOracleUpstream, expected: number): Promise<void> {
  const deadline = Date.now() + 2_000;
  while (upstream.effects < expected && Date.now() < deadline) await Bun.sleep(5);
  if (upstream.effects !== expected) throw new Error("browser broker upstream effect deadline exceeded");
}

async function runHostilePluginShard(relay: LocalBrowserRelay, uiOrigin: string): Promise<{ readonly status: number; readonly hasProof: boolean; readonly hasPort: boolean; readonly hash: string }> {
  const source = String.raw`
self.onmessage = async event => {
  const { url, uiOrigin, relaySecret } = event.data;
  let status = 0;
  try {
    const response = await fetch(url, { headers: { host: new URL(uiOrigin).host, origin: uiOrigin, referer: uiOrigin + "/", "sec-fetch-site": "same-origin", "x-semio-local-relay": relaySecret } });
    status = response.status;
  } catch {}
  self.postMessage({ status, hasProof: "localBrowserBrokerProof" in self, hasPort: "localBrowserBrokerPort" in self, hash: self.location?.hash ?? "" });
};`;
  const objectUrl = URL.createObjectURL(new Blob([source], { type: "text/javascript" }));
  const worker = new Worker(objectUrl);
  try {
    return await Promise.race([
      new Promise<{ status: number; hasProof: boolean; hasPort: boolean; hash: string }>((resolveResult, rejectResult) => {
        worker.onmessage = (event: MessageEvent) => resolveResult(event.data);
        worker.onerror = () => rejectResult(new Error("hostile plugin shard worker failed"));
        worker.postMessage({ url: `${relay.url}/_semio/hub/auth/sessions/me`, uiOrigin, relaySecret: relay.secret.toString("hex") });
      }),
      Bun.sleep(2_000).then(() => {
        throw new Error("hostile plugin shard worker deadline exceeded");
      }),
    ]);
  } finally {
    worker.terminate();
    URL.revokeObjectURL(objectUrl);
  }
}

async function proveBrowserBrokerRelay(): Promise<void> {
  const uiOrigin = "http://127.0.0.1:6066";
  const upstreamState: BrowserBrokerOracleUpstream = { effects: 0, status: 200, hold: false };
  const upstreamServer = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request): Promise<Response> {
      upstreamState.effects += 1;
      if (request.headers.get("authorization") !== `Bearer ${browserBrokerOracleEnvelope().capability}`) return new Response("unauthorized", { status: 401 });
      if (upstreamState.hold) {
        await Promise.race([new Promise<void>((resolveAbort) => request.signal.addEventListener("abort", () => resolveAbort(), { once: true })), Bun.sleep(2_000)]);
      }
      return new Response("{}", { status: upstreamState.status, headers: { "content-type": "application/json" } });
    },
  });
  const hubOrigin = `http://127.0.0.1:${upstreamServer.port}`;
  let relay: LocalBrowserRelay | undefined;
  try {
    const proof0 = randomBytes(32);
    const proof0Hex = proof0.toString("hex");
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, browserBrokerOracleEnvelope(), Buffer.from(proof0));
    const rawLocal = await browserBrokerOracleRequest(relay, uiOrigin);
    if (rawLocal.status !== 401 || upstreamState.effects !== 0 || (await rawLocal.text()).includes(proof0Hex)) throw new Error("raw local caller crossed browser broker proof boundary");
    const shardAttempt = await runHostilePluginShard(relay, uiOrigin);
    if (shardAttempt.status !== 401 || shardAttempt.hasProof || shardAttempt.hasPort || shardAttempt.hash !== "" || upstreamState.effects !== 0) throw new Error("running same-origin plugin shard crossed private broker boundary");
    const proof1 = randomBytes(32);
    const admitted = await browserBrokerOracleRequest(relay, uiOrigin, proof0Hex, proof1);
    if (!admitted.ok || admitted.headers.get("x-semio-browser-broker-advanced") !== "1" || upstreamState.effects !== 1) throw new Error("browser broker did not acknowledge an admitted ratchet");
    const replay = await browserBrokerOracleRequest(relay, uiOrigin, proof0Hex, randomBytes(32));
    if (replay.status !== 401 || upstreamState.effects !== 1 || (await replay.text()).includes(proof0Hex)) throw new Error("browser broker accepted or reflected a replayed proof");
    await relay.stop();

    upstreamState.effects = 0;
    const ttlProof = randomBytes(32);
    const ttlProofHex = ttlProof.toString("hex");
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, browserBrokerOracleEnvelope(), Buffer.from(ttlProof), 250);
    const ttlNext = randomBytes(32);
    const ttlAdvanced = await browserBrokerOracleRequest(relay, uiOrigin, ttlProofHex, ttlNext);
    if (!ttlAdvanced.ok || ttlAdvanced.headers.get("x-semio-browser-broker-advanced") !== "1" || upstreamState.effects !== 1) throw new Error("TTL oracle did not advance the initial broker proof");
    await Bun.sleep(300);
    const expired = await browserBrokerOracleRequest(relay, uiOrigin, ttlNext.toString("hex"), randomBytes(32));
    if (expired.status !== 401 || upstreamState.effects !== 1) throw new Error("expired rotated broker proof reached upstream");
    await relay.stop();

    upstreamState.effects = 0;
    const rejectedProof = randomBytes(32);
    const rejectedProofHex = rejectedProof.toString("hex");
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, browserBrokerOracleEnvelope(), Buffer.from(rejectedProof));
    upstreamState.status = 401;
    const rejectedNext = randomBytes(32);
    const rejected = await browserBrokerOracleRequest(relay, uiOrigin, rejectedProofHex, rejectedNext);
    if (rejected.status !== 401 || rejected.headers.get("x-semio-browser-broker-advanced") !== "1" || upstreamState.effects !== 1) throw new Error("upstream rejection did not close the broker epoch");
    const afterRejection = await browserBrokerOracleRequest(relay, uiOrigin, rejectedNext.toString("hex"), randomBytes(32));
    if (afterRejection.status !== 401 || upstreamState.effects !== 1) throw new Error("broker retained authority after upstream rejection");
    await relay.stop();

    upstreamState.effects = 0;
    upstreamState.status = 200;
    upstreamState.hold = true;
    const cancelProof = randomBytes(32);
    const cancelProofHex = cancelProof.toString("hex");
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, browserBrokerOracleEnvelope(), Buffer.from(cancelProof));
    const abort = new AbortController();
    const cancelled = browserBrokerOracleRequest(relay, uiOrigin, cancelProofHex, randomBytes(32), "/_semio/hub/auth/sessions/me", abort.signal).catch(() => undefined);
    await waitForBrowserBrokerEffect(upstreamState, 1);
    abort.abort();
    await cancelled;
    const replayAfterCancel = await browserBrokerOracleRequest(relay, uiOrigin, cancelProofHex, randomBytes(32));
    if (replayAfterCancel.status !== 401 || upstreamState.effects !== 1) throw new Error("cancel-after-send allowed proof reuse");
  } finally {
    await relay?.stop();
    await upstreamServer.stop(true);
  }
}

type BrowserDocumentOpenFixture = {
  readonly nowMs: number;
  readonly intent: Record<string, any>;
  readonly installedTarget: Record<string, any>;
  readonly plan: Record<string, any>;
  readonly socketGrant: Record<string, any>;
  readonly expected: { readonly httpPaths: readonly [string, string]; readonly webSocketPath: string; readonly protocol: string; readonly helloSchema: string; readonly helloPackSchemaHashByte: number; readonly responseMaxBytes: number; readonly rustWorkerBypassDenied: true; readonly scopeIsolation: { readonly left: { readonly spaceId: string; readonly documentId: string }; readonly right: { readonly spaceId: string; readonly documentId: string }; readonly leftKey: string; readonly rightKey: string; readonly localKey: string }; readonly forbiddenSocketFragments: readonly string[] };
  readonly hostile: readonly { readonly name: string; readonly stage: string; readonly replacePath?: string; readonly value?: unknown; readonly expected: string }[];
};

async function browserDocumentOpenFixture(repoRoot: string): Promise<BrowserDocumentOpenFixture> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory");
  const schema = JSON.parse(readFileSync(join(root, "🧬️browser-document-open-v1.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(root, "🌐️browser-document-open-v1.json"), "utf8")) as BrowserDocumentOpenFixture;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ allErrors: true, strict: true });
  ajv.addSchema(schema);
  const validate = ajv.getSchema(schema.$id)!;
  if (!validate(fixture)) throw new Error(`browser document-open fixture invalid: ${JSON.stringify(validate.errors)}`);
  const validatePlan = ajv.getSchema(`${schema.$id}#/$defs/plan`)!;
  for (const name of ["parent-standard-control", "parent-subset-trim"]) {
    const vector = fixture.hostile.find((row) => row.name === name)!;
    if (validatePlan(documentOpenMutation(fixture.plan, vector.replacePath!, vector.value))) throw new Error(`browser document-open schema admitted ${name}`);
  }
  return fixture;
}

/** ⚖️ Independent full-field lease relation. It is hand-written here (no production import) and
 * compares every plan-projected lease field to the installed execution target, including both
 * component digests, byte lengths, catalog generation, checkpoint and revalidation. The renderer
 * target is whatever the installation declares: a non-`react` target is a lease-path decision, not
 * a fixture constant. */
function browserDocumentOpenAuthority(plan: Record<string, any>, fixture: BrowserDocumentOpenFixture): boolean {
  const intent = fixture.intent;
  const installed = fixture.installedTarget;
  const sameJson = (left: unknown, right: unknown): boolean => JSON.stringify(left) === JSON.stringify(right);
  return installed.schema === "semio.os.document-execution-target-lease/v1"
    && installed.version === 1
    && plan.scope?.spaceId === intent.scope?.spaceId
    && plan.scope?.documentId === intent.scope?.documentId
    && sameJson(plan.scope, installed.scope)
    && plan.descriptorDigestV1 === installed.descriptorDigestV1
    && sameJson(plan.catalog, installed.catalog)
    && sameJson(plan.package, installed.package)
    && installed.component?.sha256 === installed.package?.componentSha256
    && installed.component?.blake3 === installed.package?.componentBlake3
    && installed.descriptor?.sha256 === installed.package?.descriptorByteSha256
    && Number.isSafeInteger(installed.component?.byteLength) && installed.component.byteLength >= 1 && installed.component.byteLength <= 64 * 1024 * 1024
    && Number.isSafeInteger(installed.descriptor?.byteLength) && installed.descriptor.byteLength >= 1 && installed.descriptor.byteLength <= 4 * 1024 * 1024
    && sameJson(plan.artifact, installed.artifact)
    && plan.artifact?.schema === fixture.expected.helloSchema
    && plan.artifact?.packSchemaHash === fixture.expected.helloPackSchemaHashByte.toString(16).padStart(2, "0").repeat(32)
    && sameJson(plan.parentDialect, installed.parentDialect)
    && plan.parentDialect?.artifactKind === plan.artifact?.kind
    && sameJson(plan.surface, installed.surface)
    && plan.surface?.surfaceId === intent.requestedSurfaceId
    && sameJson(plan.grant, installed.grant)
    && plan.grant?.write === (plan.surface?.role === "editor")
    && sameJson(plan.checkpoint, installed.checkpoint)
    && sameJson(plan.revalidation, installed.revalidation)
    && plan.expiresAtUnixMs > fixture.nowMs
    && plan.expiresAtUnixMs - fixture.nowMs <= 30_000;
}

async function proveBrowserDocumentOpenFixture(repoRoot: string): Promise<BrowserDocumentOpenFixture> {
  const fixture = await browserDocumentOpenFixture(repoRoot);
  const scope = fixture.intent.scope;
  const root = `/spaces/${encodeURIComponent(scope.spaceId)}/documents/${encodeURIComponent(scope.documentId)}`;
  const httpPaths = [`${root}/open-plan`, `${root}/socket-grants`];
  const webSocketPath = `${root}/socket/v1?surface=${encodeURIComponent(fixture.intent.requestedSurfaceId)}`;
  if (JSON.stringify(httpPaths) !== JSON.stringify(fixture.expected.httpPaths) || webSocketPath !== fixture.expected.webSocketPath) throw new Error("browser document-open encoded path oracle mismatch");
  if (!browserDocumentOpenAuthority(fixture.plan, fixture)) throw new Error("browser document-open plan authority oracle mismatch");
  const runtimeKey = (scope: { readonly spaceId: string; readonly documentId: string }): string => `v1:${Buffer.byteLength(scope.spaceId, "utf8")}:${Buffer.byteLength(scope.documentId, "utf8")}:${scope.spaceId}${scope.documentId}`;
  const isolation = fixture.expected.scopeIsolation;
  const localKey = `local:v1:${Buffer.byteLength(isolation.left.documentId, "utf8")}:${isolation.left.documentId}`;
  if (runtimeKey(isolation.left) !== isolation.leftKey || runtimeKey(isolation.right) !== isolation.rightKey || localKey !== isolation.localKey || isolation.leftKey === isolation.rightKey || isolation.leftKey === isolation.localKey) throw new Error("browser document-open scope-key oracle mismatch");
  const exchange = { schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: fixture.plan.receipt };
  if (Object.keys(exchange).join(",") !== "schema,version,planReceipt" || fixture.socketGrant.expiresAtMs > fixture.plan.expiresAtUnixMs || fixture.socketGrant.protocol !== fixture.expected.protocol) throw new Error("browser document-open receipt exchange oracle mismatch");
  const hello = { SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: fixture.plan.artifact.schema, pack_schema_hash: new Array(32).fill(fixture.expected.helloPackSchemaHashByte), resume_token: null, frontier: null } };
  const publicTransport = JSON.stringify({ webSocketPath, protocol: fixture.socketGrant.protocol, hello });
  for (const fragment of fixture.expected.forbiddenSocketFragments) if (publicTransport.includes(fragment)) throw new Error(`browser document-open public transport leaked ${fragment}`);
  if (fixture.expected.rustWorkerBypassDenied !== true) throw new Error("browser document-open execution ownership oracle drift");
  let hostile = 0;
  for (const vector of fixture.hostile) {
    if (vector.expected === "invalid-plan") {
      const candidate = documentOpenMutation(fixture.plan, vector.replacePath!, vector.value);
      if (browserDocumentOpenAuthority(candidate, fixture)) throw new Error(`browser document-open authority oracle admitted ${vector.name}`);
    } else if (vector.expected === "response-too-large") {
      if (vector.value !== fixture.expected.responseMaxBytes + 1) throw new Error("browser document-open max-plus-one oracle drift");
    } else if (vector.expected === "cancelled-before-exchange") {
      if (vector.stage !== "exchange" || httpPaths[0] === httpPaths[1]) throw new Error("browser document-open cancellation sequence oracle drift");
    } else if (vector.expected === "credential-free") {
      if (fixture.expected.forbiddenSocketFragments.some((fragment) => webSocketPath.includes(fragment))) throw new Error("browser document-open socket URL is credential-bearing");
    } else if (vector.expected === "withheld-activation") {
      if (vector.stage !== "session" || vector.value === fixture.socketGrant.actorId) throw new Error("browser document-open mismatched Session activation oracle drift");
    } else {
      throw new Error(`browser document-open unknown hostile outcome ${vector.expected}`);
    }
    hostile += 1;
  }
  console.log(`browser-document-open-oracle: ajv=1 paths=3 installed-target=1 scope-keys=2 authority=1 exchange=1 websocket=1 rust-worker-bypass=denied hostile=${hostile} bound=${fixture.expected.responseMaxBytes} redaction=${fixture.expected.forbiddenSocketFragments.length} passed`);
  return fixture;
}

async function proveBrowserDocumentOpenRuntime(repoRoot: string, fixture: BrowserDocumentOpenFixture): Promise<void> {
  const current = structuredClone(fixture);
  current.plan.expiresAtUnixMs = Date.now() + 30_000;
  current.socketGrant.expiresAtMs = Date.now() + 25_000;
  const effects = { open: 0, exchange: 0, socket: 0, hello: 0 };
  let documentSocket: { send(data: Uint8Array): void } | undefined;
  const capability = `session.v1.${"a".repeat(32)}.${"b".repeat(64)}`;
  const authorityServer = Bun.serve<{ admitted: boolean }>({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request, server): Promise<Response | undefined> {
      const url = new URL(request.url);
      if (request.method === "POST" && [current.expected.httpPaths[0], current.expected.httpPaths[1]].includes(url.pathname)) {
        if (request.headers.get("authorization") !== `Bearer ${capability}` || request.headers.get("content-type") !== "application/json" || url.search !== "") return new Response("", { status: 401 });
        const bytes = new Uint8Array(await request.arrayBuffer());
        try {
          if (bytes.byteLength === 0 || bytes.byteLength > 8 * 1024) return new Response("", { status: 413 });
          const value = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
          if (url.pathname === current.expected.httpPaths[0]) {
            const intent = value as Record<string, any>;
            const clientInstanceBytes = typeof intent.clientInstanceId === "string" ? Buffer.byteLength(intent.clientInstanceId, "utf8") : 0;
            if (JSON.stringify(Object.keys(intent).sort()) !== JSON.stringify(["clientInstanceId", "requestedSurfaceId", "schema", "scope", "version"]) || intent.schema !== current.intent.schema || intent.version !== current.intent.version || JSON.stringify(intent.scope) !== JSON.stringify(current.intent.scope) || intent.requestedSurfaceId !== current.intent.requestedSurfaceId || clientInstanceBytes === 0 || clientInstanceBytes > 128 || /[\u0000-\u001f\u007f]/u.test(intent.clientInstanceId)) return new Response("", { status: 400 });
            effects.open += 1;
            return Response.json(current.plan, { headers: { "cache-control": "no-store" } });
          }
          if (JSON.stringify(value) !== JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: current.plan.receipt })) return new Response("", { status: 400 });
          effects.exchange += 1;
          return Response.json(current.socketGrant, { headers: { "cache-control": "no-store" } });
        } finally {
          bytes.fill(0);
        }
      }
      if (request.method === "GET" && `${url.pathname}${url.search}` === current.expected.webSocketPath) {
        const protocols = (request.headers.get("sec-websocket-protocol") ?? "").split(",").map((value) => value.trim());
        if (protocols.length !== 2 || protocols[0] !== current.expected.protocol || protocols[1] !== current.socketGrant.grant || request.headers.has("authorization")) return new Response("", { status: 401 });
        effects.socket += 1;
        return server.upgrade(request, { data: { admitted: true }, headers: { "Sec-WebSocket-Protocol": current.expected.protocol } }) ? undefined : new Response("", { status: 500 });
      }
      return new Response("", { status: 404 });
    },
    websocket: {
      message(socket, message): void {
        if (!socket.data.admitted || effects.hello !== 0) return;
        const bytes = typeof message === "string" ? new TextEncoder().encode(message) : new Uint8Array(message);
        const decoded = decodeClientFrame(bytes);
        if (typeof decoded.frame === "string" || !("SocketHelloV1" in decoded.frame)) return;
        const hello = decoded.frame.SocketHelloV1;
        if (hello.schema !== current.expected.helloSchema || hello.pack_schema_hash.length !== 32 || hello.pack_schema_hash.some((byte) => byte !== current.expected.helloPackSchemaHashByte) || hello.resume_token !== null || hello.frontier !== null) return;
        effects.hello += 1;
        documentSocket = socket;
        const frontier = { document_id: current.intent.scope.documentId, head_edit_ordinal: 0, head_edit_id: "", last_commit_seq: 0, chain_hash: new Array(32).fill(0) };
        socket.send(encodeServerFrame({ Welcome: { session_id: "browser-open-session", resume_token: "browser-open-resume", server_frontier: frontier, bootstrap: "None" } }, "command"));
      },
    },
  });
  const hubOrigin = `http://127.0.0.1:${authorityServer.port}`;
  const openWire = Array.from(encodeBackboneWorkerRequest({
    kind: "open",
    documentId: current.intent.scope.documentId,
    schema: current.expected.helloSchema,
    bindings: [{ kind: "hub", baseUrl: hubOrigin, spaceId: current.intent.scope.spaceId, installedTarget: current.installedTarget as unknown as import("@semio-tech/framework-os").DocumentExecutionTargetLeaseFieldsV1 }],
    actor: "browser-untrusted-actor",
    packSchemaHash: new Array(32).fill(current.expected.helloPackSchemaHashByte),
  }));
  const bootstrapProof = randomBytes(32);
  let proofHex = "";
  let relay: LocalBrowserRelay | undefined;
  let viteServer: { close(): Promise<void> } | undefined;
  let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
  const browserDiagnostics: string[] = [];
  const priorViteEnvironment = {
    S_OS_PORT: process.env.S_OS_PORT,
    S_HUB_URL: process.env.S_HUB_URL,
    S_LOCAL_RELAY_URL: process.env.S_LOCAL_RELAY_URL,
    S_LOCAL_RELAY_SECRET: process.env.S_LOCAL_RELAY_SECRET,
    SEMIO_PLUGIN: process.env.SEMIO_PLUGIN,
    SEMIO_RENDERER: process.env.SEMIO_RENDERER,
  };
  try {
    const uiPort = await freeLoopbackPort();
    const uiOrigin = `http://127.0.0.1:${uiPort}`;
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, { schema: "semio.hub.local-credential-envelope/v1", clientClass: "react-relay", capability }, bootstrapProof);
    process.env.S_OS_PORT = String(uiPort);
    process.env.S_HUB_URL = hubOrigin;
    process.env.S_LOCAL_RELAY_URL = relay.url;
    process.env.S_LOCAL_RELAY_SECRET = relay.secret.toString("hex");
    process.env.SEMIO_PLUGIN = "s";
    process.env.SEMIO_RENDERER = "react";
    const { createServer: createViteServer } = await import("vite");
    viteServer = await createViteServer({
      configFile: join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts"),
      server: { host: "127.0.0.1", port: uiPort, strictPort: true },
      clearScreen: false,
    });
    await (viteServer as { listen(): Promise<void> }).listen();
    const { chromium } = await import("playwright");
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    page.on("console", (message) => browserDiagnostics.push(`console:${message.type()}:${message.text()}`));
    page.on("pageerror", (error) => browserDiagnostics.push(`pageerror:${error.message}`));
    page.on("requestfailed", (request) => browserDiagnostics.push(`requestfailed:${request.url()}:${request.failure()?.errorText ?? "unknown"}`));
    page.on("response", (response) => {
      if (response.url().includes("/_semio/hub/")) browserDiagnostics.push(`response:${response.status()}:${response.url()}`);
    });
    await page.goto(uiOrigin, { waitUntil: "domcontentloaded", timeout: 10_000 });
    await page.setContent("<!doctype html><meta charset=utf-8>");
    current.plan.expiresAtUnixMs = Date.now() + 30_000;
    current.socketGrant.expiresAtMs = Date.now() + 25_000;
    const relayPort = Number(new URL(relay.url).port);
    const relaySecret = Buffer.from(relay.secret);
    await relay.stop();
    const liveProof = randomBytes(32);
    proofHex = liveProof.toString("hex");
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, { schema: "semio.hub.local-credential-envelope/v1", clientClass: "react-relay", capability }, liveProof, BROWSER_BROKER_PROOF_TTL_MS, { port: relayPort, secret: relaySecret });
    await page.evaluate(({ workerUrl, proof, openWire }) => {
      const state = (globalThis as any).__semio = { messages: [], errors: [], started: false };
      history.replaceState(history.state, "", `${location.pathname}${location.search}`);
      const worker = new Worker(workerUrl, { type: "module" });
      const channel = new MessageChannel();
      worker.postMessage({ kind: "semio-browser-broker-port", port: channel.port2 }, [channel.port2]);
      channel.port1.onmessage = (event) => {
        if (event.data?.kind !== "initialized" || state.started) return;
        if (event.data.ok !== true) {
          state.errors.push("broker bootstrap rejected");
          return;
        }
        state.started = true;
        worker.postMessage({ wire: new Uint8Array(openWire) });
      };
      channel.port1.start();
      channel.port1.postMessage({ kind: "initialize", proof });
      worker.onerror = (event) => state.errors.push(String(event.message ?? "worker error"));
      worker.onmessage = (event) => {
        state.messages.push(event.data?.wire ? Array.from(event.data.wire) : event.data);
      };
      state.worker = worker;
    }, { workerUrl: `/@fs${join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts")}`, proof: proofHex, openWire });
    await page.waitForFunction(() => (globalThis as any).__semio?.errors?.length > 0 || (globalThis as any).__semio?.messages?.length > 1, undefined, { timeout: 10_000 });
    const deadline = Date.now() + 10_000;
    let browserState = await page.evaluate(() => ({ hash: location.hash, errors: (globalThis as any).__semio?.errors ?? [], messages: (globalThis as any).__semio?.messages ?? [] }));
    let activation = browserState.messages.flatMap((wire: number[]) => {
      try {
        const message = decodeBackboneWorkerResponse(Uint8Array.from(wire));
        return message.kind === "socket-actor" ? [message] : [];
      } catch {
        return [];
      }
    });
    while (effects.hello !== 1 && Date.now() < deadline) {
      await Bun.sleep(10);
      browserState = await page.evaluate(() => ({ hash: location.hash, errors: (globalThis as any).__semio?.errors ?? [], messages: (globalThis as any).__semio?.messages ?? [] }));
      activation = browserState.messages.flatMap((wire: number[]) => {
        try {
          const message = decodeBackboneWorkerResponse(Uint8Array.from(wire));
          return message.kind === "socket-actor" ? [message] : [];
        } catch {
          return [];
        }
      });
    }
    if (effects.hello !== 1 || activation.length !== 0 || !documentSocket) throw new Error(`browser document-open activated before authenticated Session effects=${JSON.stringify(effects)} activation=${activation.length}`);
    documentSocket.send(encodeServerFrame({ Session: { actor: current.socketGrant.actorId, color: 7 } }, "command"));
    while (activation.length !== 1 && Date.now() < deadline) {
      await Bun.sleep(10);
      browserState = await page.evaluate(() => ({ hash: location.hash, errors: (globalThis as any).__semio?.errors ?? [], messages: (globalThis as any).__semio?.messages ?? [] }));
      activation = browserState.messages.flatMap((wire: number[]) => {
        try {
          const message = decodeBackboneWorkerResponse(Uint8Array.from(wire));
          return message.kind === "socket-actor" ? [message] : [];
        } catch {
          return [];
        }
      });
    }
    if (effects.open !== 1 || effects.exchange !== 1 || effects.socket !== 1 || effects.hello !== 1 || activation.length !== 1 || activation[0]!.actorId !== current.socketGrant.actorId || browserState.hash !== "" || browserState.errors.length !== 0 || browserState.messages.length < 2)
      throw new Error(`browser document-open runtime mismatch effects=${JSON.stringify(effects)} state=${JSON.stringify(browserState)} diagnostics=${browserDiagnostics.slice(-8).join("|")}`);
    console.log("browser-document-open-runtime: chromium-worker=1 authenticated-open=1 receipt-exchange=1 credential-free-websocket=1 authoritative-tag7=1 pre-session-activation=0 matched-session-activation=1 fragment-cleared=1 passed");
  } finally {
    await browser?.close().catch(() => undefined);
    await viteServer?.close().catch(() => undefined);
    await relay?.stop().catch(() => undefined);
    await authorityServer.stop(true);
    for (const [key, value] of Object.entries(priorViteEnvironment)) {
      if (value === undefined) delete process.env[key];
      else process.env[key] = value;
    }
  }
}

function adminRelayOracleEnvelope(capability: string): Record<string, unknown> {
  return { schema: "semio.hub.local-credential-envelope/v1", clientClass: "admin-relay", capability, expiresAt: Date.now() + 60_000 };
}

async function adminRelayBootstrap(relay: LocalAdminRelay, proof: string): Promise<Response> {
  return fetch(`${relay.url}/__semio/admin/bootstrap`, {
    method: "POST",
    headers: { origin: relay.url, referer: `${relay.url}/admin/`, "sec-fetch-site": "same-origin", "x-semio-admin-bootstrap": proof },
    redirect: "error",
  });
}

async function proveAdminRelayBoundary(repoRoot: string): Promise<void> {
  const capability = `session.v1.${"3".repeat(32)}.${"4".repeat(64)}`;
  const upstream = {
    effects: 0,
    unsafeEffects: 0,
    staticAuthorization: "",
    status: 200,
    responseBytes: 0,
    blocked: false,
    started: 0,
    aborted: 0,
    releases: new Set<() => void>(),
    staticResponseBytes: 0,
    staticBlocked: false,
    staticStarted: 0,
    staticAborted: 0,
    staticReleases: new Set<() => void>(),
  };
  const upstreamServer = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request): Promise<Response> {
      const url = new URL(request.url);
      const staticRequest = url.pathname.startsWith("/admin/") && !url.pathname.startsWith("/admin/api/");
      if (staticRequest) {
        upstream.staticAuthorization = request.headers.get("authorization") ?? "";
      } else {
        upstream.effects += 1;
        if (request.method === "POST") upstream.unsafeEffects += 1;
        if (request.headers.get("authorization") !== `Bearer ${capability}`) return new Response("unauthorized", { status: 401 });
      }
      if (staticRequest ? upstream.staticBlocked : upstream.blocked) {
        if (staticRequest) upstream.staticStarted += 1;
        else upstream.started += 1;
        await new Promise<void>((resolveBlocked) => {
          const finish = (): void => {
            request.signal.removeEventListener("abort", abort);
            (staticRequest ? upstream.staticReleases : upstream.releases).delete(finish);
            resolveBlocked();
          };
          const abort = (): void => {
            if (staticRequest) upstream.staticAborted += 1;
            else upstream.aborted += 1;
            finish();
          };
          (staticRequest ? upstream.staticReleases : upstream.releases).add(finish);
          request.signal.addEventListener("abort", abort, { once: true });
        });
      }
      const responseBytes = staticRequest ? upstream.staticResponseBytes : upstream.responseBytes;
      if (responseBytes > 0) return new Response(new Uint8Array(responseBytes), { status: upstream.status, headers: { "content-type": "application/octet-stream" } });
      if (staticRequest) return new Response("<main>admin</main>", { headers: { "content-type": "text/html" } });
      return new Response(JSON.stringify({ ok: true }), { status: upstream.status, headers: { "content-type": "application/json" } });
    },
  });
  const hubOrigin = `http://127.0.0.1:${upstreamServer.port}`;
  let relay: LocalAdminRelay | undefined;
  try {
    const proof = randomBytes(32);
    const proofHex = proof.toString("hex");
    relay = startLocalAdminRelay(hubOrigin, adminRelayOracleEnvelope(capability), Buffer.from(proof));
    const staticResponse = await fetch(`${relay.url}/admin/`);
    if (!staticResponse.ok || upstream.staticAuthorization !== "" || (await staticResponse.text()) !== "<main>admin</main>") throw new Error("admin relay static shell carried administrator authority");
    upstream.staticResponseBytes = LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES;
    const exactStatic = await fetch(`${relay.url}/admin/static-exact.js`);
    if (!exactStatic.ok || (await exactStatic.arrayBuffer()).byteLength !== LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES) throw new Error("admin relay rejected its exact static response ceiling");
    upstream.staticResponseBytes = LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES + 1;
    const oversizedStatic = await fetch(`${relay.url}/admin/static-oversized.js`);
    upstream.staticResponseBytes = 0;
    if (oversizedStatic.status !== 503) throw new Error("admin relay static response max+1 law failed");
    const rawLocal = await fetch(`${relay.url}/admin/api/overview`);
    if (rawLocal.status !== 401 || upstream.effects !== 0 || (await rawLocal.text()).includes(capability)) throw new Error("raw local caller crossed the admin relay cookie boundary");
    const bootstrapped = await adminRelayBootstrap(relay, proofHex);
    const setCookie = bootstrapped.headers.get("set-cookie") ?? "";
    const cookie = setCookie.match(new RegExp(`^${ADMIN_RELAY_COOKIE}=([0-9a-f]{64});`, "u"))?.[1];
    if (
      bootstrapped.status !== 204 ||
      !cookie ||
      !setCookie.includes("HttpOnly") ||
      !setCookie.includes("SameSite=Strict") ||
      !setCookie.includes("Path=/") ||
      /(?:^|;)\s*Domain=/iu.test(setCookie) ||
      setCookie.includes(capability) ||
      (await bootstrapped.text()).includes(proofHex)
    )
      throw new Error("admin relay did not issue an opaque host-only HttpOnly strict cookie");
    const replay = await adminRelayBootstrap(relay, proofHex);
    if (replay.status !== 401 || upstream.effects !== 0) throw new Error("admin relay replayed its one-use fragment bootstrap");
    const admitted = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const admittedText = await admitted.text();
    if (!admitted.ok || admittedText !== '{"ok":true}' || admittedText.includes(capability) || upstream.effects !== 1) throw new Error("admin relay did not proxy an opaque-cookie request with memory-only authority");
    const intentBody = JSON.stringify({ kind: "rebuild-directory-projections", requestId: "relay-oracle:rebuild", expectedHeadSeq: 0 });
    const forgedUnsafe = await fetch(`${relay.url}/admin/api/intents`, { method: "POST", headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}`, "content-type": "application/json" }, body: intentBody });
    if (forgedUnsafe.status !== 401 || upstream.unsafeEffects !== 0) throw new Error("admin relay admitted a cross-origin unsafe request");
    const admittedUnsafe = await fetch(`${relay.url}/admin/api/intents`, {
      method: "POST",
      headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}`, origin: relay.url, referer: `${relay.url}/admin/`, "sec-fetch-site": "same-origin", "content-type": "application/json" },
      body: intentBody,
    });
    if (!admittedUnsafe.ok || upstream.unsafeEffects !== 1) throw new Error("admin relay rejected its same-origin unsafe request");
    const paged = await fetch(`${relay.url}/admin/api/connections?limit=100`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!paged.ok || upstream.effects !== 3) throw new Error("admin relay rejected an exact bounded page query");
    const documents = await fetch(`${relay.url}/admin/api/documents?limit=100&space=studio`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!documents.ok || upstream.effects !== 4) throw new Error("admin relay rejected an exact bounded document projection query");
    const spaces = await fetch(`${relay.url}/admin/api/spaces?limit=100`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!spaces.ok || upstream.effects !== 5) throw new Error("admin relay rejected an exact bounded space projection query");
    const space = await fetch(`${relay.url}/admin/api/spaces/studio?limit=100`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!space.ok || upstream.effects !== 6) throw new Error("admin relay rejected an exact bounded space-member projection query");
    const arbitraryQuery = await fetch(`${relay.url}/admin/api/connections?limit=100&scope=all`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const unboundedDocuments = await fetch(`${relay.url}/admin/api/documents?space=studio`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const unboundedSpaces = await fetch(`${relay.url}/admin/api/spaces`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const unboundedSpace = await fetch(`${relay.url}/admin/api/spaces/studio`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const legacyMutation = await fetch(`${relay.url}/admin/api/directory/rebuild`, {
      method: "POST",
      headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}`, origin: relay.url, referer: `${relay.url}/admin/`, "sec-fetch-site": "same-origin" },
    });
    if (arbitraryQuery.status !== 404 || unboundedDocuments.status !== 404 || unboundedSpaces.status !== 404 || unboundedSpace.status !== 404 || legacyMutation.status !== 404 || upstream.effects !== 6)
      throw new Error("admin relay admitted an arbitrary, unbounded, or legacy request");

    const sameOriginHeaders = { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}`, origin: relay.url, referer: `${relay.url}/admin/`, "sec-fetch-site": "same-origin", "content-type": "application/json" };
    const beforeLimits = upstream.effects;
    const exactBody = await fetch(`${relay.url}/admin/api/intents`, { method: "POST", headers: sameOriginHeaders, body: new Uint8Array(8 * 1024) });
    const oversizedBody = await fetch(`${relay.url}/admin/api/intents`, { method: "POST", headers: sameOriginHeaders, body: new Uint8Array(8 * 1024 + 1) });
    if (!exactBody.ok || oversizedBody.status !== 413 || upstream.effects !== beforeLimits + 1) throw new Error("admin relay request-body max+1 law failed");
    upstream.responseBytes = 64 * 1024;
    const exactResponse = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!exactResponse.ok || (await exactResponse.arrayBuffer()).byteLength !== 64 * 1024) throw new Error("admin relay rejected its exact response ceiling");
    upstream.responseBytes = 64 * 1024 + 1;
    const oversizedResponse = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    upstream.responseBytes = 0;
    if (oversizedResponse.status !== 503 || upstream.effects !== beforeLimits + 3) throw new Error("admin relay response max+1 law failed");

    upstream.blocked = true;
    upstream.started = 0;
    const abortController = new AbortController();
    const abortedRequest = fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` }, signal: abortController.signal }).catch(() => undefined);
    for (let attempt = 0; upstream.started !== 1 && attempt < 200; attempt += 1) await Bun.sleep(5);
    if (upstream.started !== 1) throw new Error("admin relay downstream-abort law did not reach upstream");
    const abortedBefore = upstream.aborted;
    abortController.abort();
    await abortedRequest;
    for (let attempt = 0; upstream.aborted === abortedBefore && attempt < 200; attempt += 1) await Bun.sleep(5);
    if (upstream.aborted !== abortedBefore + 1) throw new Error("admin relay did not propagate downstream cancellation upstream");

    upstream.staticBlocked = true;
    upstream.staticStarted = 0;
    const staticAbortController = new AbortController();
    const abortedStatic = fetch(`${relay.url}/admin/static-abort.js`, { signal: staticAbortController.signal }).catch(() => undefined);
    for (let attempt = 0; upstream.staticStarted !== 1 && attempt < 200; attempt += 1) await Bun.sleep(5);
    const staticAbortedBefore = upstream.staticAborted;
    staticAbortController.abort();
    await abortedStatic;
    for (let attempt = 0; upstream.staticAborted === staticAbortedBefore && attempt < 200; attempt += 1) await Bun.sleep(5);
    upstream.staticBlocked = false;
    if (upstream.staticStarted !== 1 || upstream.staticAborted !== staticAbortedBefore + 1) throw new Error("admin relay did not propagate static downstream cancellation upstream");

    upstream.started = 0;
    const saturated = Array.from({ length: LOCAL_RELAY_MAX_IN_FLIGHT }, () => fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } }));
    for (let attempt = 0; upstream.started !== LOCAL_RELAY_MAX_IN_FLIGHT && attempt < 400; attempt += 1) await Bun.sleep(5);
    if (upstream.started !== LOCAL_RELAY_MAX_IN_FLIGHT) throw new Error("admin relay did not fill its exact in-flight boundary");
    const overflow = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const staticOverflow = await fetch(`${relay.url}/admin/static-overflow.js`);
    if (overflow.status !== 503 || staticOverflow.status !== 503) throw new Error("admin relay admitted max+1 API or static request outside its aggregate boundary");
    upstream.blocked = false;
    for (const release of [...upstream.releases]) release();
    if ((await Promise.all(saturated)).some((response) => !response.ok)) throw new Error("admin relay rejected an admitted in-flight request");

    upstream.staticBlocked = true;
    upstream.staticStarted = 0;
    const stoppedRequest = fetch(`${relay.url}/admin/static-stop.js`).catch(() => undefined);
    for (let attempt = 0; upstream.staticStarted !== 1 && attempt < 200; attempt += 1) await Bun.sleep(5);
    const stopAbortedBefore = upstream.staticAborted;
    await relay.stop();
    await stoppedRequest;
    if (upstream.staticStarted !== 1 || upstream.staticAborted !== stopAbortedBefore + 1)
      throw new Error(`admin relay stop did not cancel active static upstream ownership: started=${upstream.staticStarted} aborted=${upstream.staticAborted} before=${stopAbortedBefore}`);
    upstream.staticBlocked = false;

    upstream.effects = 0;
    const expiringProof = randomBytes(32);
    const expiringProofHex = expiringProof.toString("hex");
    relay = startLocalAdminRelay(hubOrigin, adminRelayOracleEnvelope(capability), Buffer.from(expiringProof), 250, 250);
    const expiringBootstrap = await adminRelayBootstrap(relay, expiringProofHex);
    const expiringCookie = (expiringBootstrap.headers.get("set-cookie") ?? "").match(new RegExp(`^${ADMIN_RELAY_COOKIE}=([0-9a-f]{64});`, "u"))?.[1];
    if (!expiringCookie) throw new Error("admin relay expiry oracle lacked its opaque cookie");
    await Bun.sleep(300);
    const expired = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${expiringCookie}` } });
    if (expired.status !== 401 || upstream.effects !== 0) throw new Error("expired admin relay cookie reached upstream");

    const sessionSource = readFileSync(join(repoRoot, "🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx"), "utf8");
    const viteSource = readFileSync(join(repoRoot, "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/⚙️vite.config.ts"), "utf8");
    const launch = readFileSync(join(repoRoot, ".vscode/🧩️launch.seed.jsonc"), "utf8");
    if (sessionSource.includes("sessionStorage") || sessionSource.includes("headers.authorization") || sessionSource.includes("Bearer ${") || !sessionSource.includes("#semio-admin=") || !sessionSource.includes('credentials: "same-origin"'))
      throw new Error("admin SPA regained a browser-owned bearer carrier");
    if (viteSource.includes('"/admin/api": hubProxy') || !launch.includes("os-hub:dev-secure-admin")) throw new Error("admin relay dev/launch ownership drift");
  } finally {
    await relay?.stop();
    await upstreamServer.stop(true);
  }
}

/** 🧬 Validates the literal cross-language journey before it may drive a real process/browser. */
async function adminLiveJourneyFixture(repoRoot: string): Promise<AdminLiveJourneyFixture> {
  const fixtureRoot = join(repoRoot, "🌎️hub/📇️directory/🧫️fixtures/🚶️admin-live-journey-v1");
  const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as AdminLiveJourneyFixture;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`admin live journey fixture invalid: ${JSON.stringify(validate.errors)}`);
  const admissionRoot = join(repoRoot, "🌎️hub/🚀️local-bootstrap/🧪️fixtures/⏳️idle-admission-v1");
  const admissionSchema = JSON.parse(readFileSync(join(admissionRoot, "🧬️.schema.json"), "utf8"));
  const admission = JSON.parse(readFileSync(join(admissionRoot, "🔣️.json"), "utf8")) as { exchangeDeadlineMs: number; idleBeforeAdmissionMs: number; frameHex: string; payloadHex: string };
  const validateAdmission = new Ajv2020({ allErrors: true, strict: true }).compile(admissionSchema);
  if (!validateAdmission(admission)) throw new Error(`local bootstrap idle-admission fixture invalid: ${JSON.stringify(validateAdmission.errors)}`);
  const frame = Buffer.from(admission.frameHex, "hex");
  const payload = Buffer.from(admission.payloadHex, "hex");
  if (admission.idleBeforeAdmissionMs <= admission.exchangeDeadlineMs || frame.readUInt32BE(0) !== payload.byteLength || !frame.subarray(4).equals(payload)) throw new Error("local bootstrap idle/admitted-frame oracle mismatch");
  const locales = fixture.languages.map(({ locale }) => locale).sort().join(",");
  if (locales !== "de,en" || fixture.languages[0]?.overview === fixture.languages[1]?.overview) throw new Error("admin live journey bilingual inventory drift");
  if (Buffer.byteLength(JSON.stringify(fixture.mutation), "utf8") > 8 * 1024 || Buffer.byteLength(JSON.stringify(fixture.operation), "utf8") > 8 * 1024) throw new Error("admin live journey intent exceeded relay bound");
  console.log("admin-live-journey fixture: AJV 2/2; idle/admitted frame 2/2; bilingual inventory 2/2; bounded intents 2/2");
  return fixture;
}

/** 🎭 Exercises the protected relay, real SQLite hub and shipped SPA in one bounded Chromium journey. */
async function proveAdminLiveJourney(repoRoot: string, root: string, fixture: AdminLiveJourneyFixture): Promise<void> {
  const profile: LocalProfile = { ...fixture.profile, allowedClientClasses: ["admin-relay"] };
  const deadline = Date.now() + fixture.limits.journeyMs;
  const remaining = (): number => {
    const value = deadline - Date.now();
    if (value <= 0) throw new Error("admin live journey deadline exceeded");
    return value;
  };
  let run: LocalHubRun | undefined;
  let relay: LocalAdminRelay | undefined;
  let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
  const browserDiagnostics: string[] = [];
  const retainBrowserDiagnostic = (value: string): void => {
    browserDiagnostics.push(value.slice(0, 512));
    if (browserDiagnostics.length > 16) browserDiagnostics.shift();
  };
  try {
    run = await startLocalHub(repoRoot, root, [profile], {
      capture: true,
      isolatedSecuritySmoke: true,
      adminSubjects: [`semio.local.bootstrap/v1:${fixture.profile.subject}`],
    });
    const readiness = await waitForReadiness(run, true);
    if (readiness.directory?.ready !== true) throw new Error("admin live journey directory was not ready");
    const envelope = await issueLocalCredential(run, fixture.profile.profileId, "admin-relay");
    const proof = randomBytes(32);
    const proofHex = proof.toString("hex");
    relay = startLocalAdminRelay(`http://127.0.0.1:${run.port}`, envelope, proof);
    const { chromium } = await import("playwright");
    browser = await chromium.launch({ headless: true });
    const context = await browser.newContext({ locale: "fr-FR" });
    const page = await context.newPage();
    page.on("console", (message) => retainBrowserDiagnostic(`console:${message.type()}:${message.text()}`));
    page.on("pageerror", (error) => retainBrowserDiagnostic(`pageerror:${error.message}`));
    page.on("requestfailed", (request) => retainBrowserDiagnostic(`requestfailed:${request.url()}:${request.failure()?.errorText ?? "unknown"}`));
    page.on("response", (response) => {
      if (response.status() >= 400) retainBrowserDiagnostic(`response:${response.status()}:${response.url()}`);
    });
    page.setDefaultTimeout(Math.min(10_000, remaining()));
    await page.goto(`${relay.url}/admin/#semio-admin=${proofHex}`, { waitUntil: "domcontentloaded", timeout: remaining() });
    await page.getByRole("dialog", { name: "Language · Sprache" }).waitFor();
    await page.getByRole("button", { name: "English", exact: true }).click();
    await page.locator("#admin-tab-overview").waitFor();
    if (new URL(page.url()).hash !== "") throw new Error("admin live journey retained the one-use bootstrap fragment");

    const request = async (method: "GET" | "POST", path: string, body?: unknown): Promise<{ status: number; bytes: number; value: any }> => {
      const result = await page.evaluate(
        async ({ method, path, body }) => {
          const response = await fetch(path, {
            method,
            credentials: "same-origin",
            headers: body === undefined ? undefined : { "content-type": "application/json" },
            body: body === undefined ? undefined : JSON.stringify(body),
            signal: AbortSignal.timeout(2_000),
          });
          const text = await response.text();
          return { status: response.status, bytes: new TextEncoder().encode(text).byteLength, text };
        },
        { method, path, body },
      );
      if (result.bytes > fixture.limits.responseBytes) throw new Error(`admin live journey response exceeded ${fixture.limits.responseBytes} bytes`);
      let value: any;
      try {
        value = result.text.length === 0 ? undefined : JSON.parse(result.text);
      } catch {
        throw new Error(`admin live journey ${method} ${path} returned non-JSON status ${result.status}`);
      }
      return { status: result.status, bytes: result.bytes, value };
    };

    const english = fixture.languages.find(({ locale }) => locale === "en")!;
    if ((await page.locator("#admin-tab-overview").textContent())?.trim() !== english.overview || (await page.locator("#admin-tab-spaces").textContent())?.trim() !== english.spaces) throw new Error("admin live journey English navigation mismatch");
    const overview = await request("GET", "/admin/api/overview");
    if (overview.status !== 200 || overview.value?.backends?.sqlite !== true || !Number.isSafeInteger(overview.value?.headSeq)) throw new Error("admin live journey bounded SQLite overview mismatch");

    const created = await request("POST", "/admin/api/intents", fixture.mutation);
    if (created.status !== 200 || created.value?.state !== "succeeded" || created.value?.outcome?.code !== "directory-events-appended" || created.value?.outcome?.durable !== true) throw new Error("admin live journey create-space receipt mismatch");
    if (!existsSync(join(run.runRoot, "data", "directory.db"))) throw new Error("admin live journey did not durably materialize the SQLite directory");
    await page.locator("#admin-tab-spaces").click();
    await page.getByText(fixture.mutation.name, { exact: true }).waitFor();
    if ((await page.locator("#admin-space-create-open").textContent())?.trim() !== english.newSpace) throw new Error("admin live journey English mutation surface mismatch");

    const current = await request("GET", "/admin/api/overview");
    const rebuild = await request("POST", "/admin/api/intents", { ...fixture.operation, expectedHeadSeq: current.value?.headSeq });
    if (rebuild.status !== 202 || rebuild.value?.state !== "accepted" || typeof rebuild.value?.operationId !== "string") throw new Error("admin live journey rebuild acceptance mismatch");
    const operationPath = `/admin/api/operations/${encodeURIComponent(rebuild.value.operationId)}`;
    const cancellation = await request("POST", `${operationPath}/cancel`);
    if (cancellation.status !== 200 && cancellation.status !== 409) throw new Error("admin live journey cancellation admission mismatch");
    if (cancellation.status === 200 && cancellation.value?.progress?.cancelRequested !== true) throw new Error("admin live journey cancellation was not observable");
    let completed = -1;
    let terminal: any;
    while (Date.now() < deadline) {
      const status = await request("GET", operationPath);
      if (status.status !== 200 || typeof status.value?.receipt?.state !== "string") throw new Error("admin live journey operation polling mismatch");
      const progress = status.value.progress;
      if (progress) {
        if (!Number.isSafeInteger(progress.completedEvents) || progress.completedEvents < completed || progress.completedEvents > progress.totalEvents) throw new Error("admin live journey operation progress was not bounded monotonic state");
        completed = progress.completedEvents;
      }
      if (status.value.receipt.state !== "accepted") {
        terminal = status.value.receipt;
        break;
      }
      await Bun.sleep(Math.min(fixture.limits.pollMs, remaining()));
    }
    if (!terminal || (terminal.state !== "cancelled" && terminal.state !== "succeeded") || (cancellation.status === 200 && terminal.state !== "cancelled"))
      throw new Error(`admin live journey operation lacked a valid terminal: cancel=${cancellation.status}:${JSON.stringify(cancellation.value)} terminal=${JSON.stringify(terminal)}`);

    await page.locator("#admin-locale-switch").click();
    await page.getByRole("option", { name: "DE", exact: true }).click();
    const german = fixture.languages.find(({ locale }) => locale === "de")!;
    if ((await page.locator("#admin-tab-overview").textContent())?.trim() !== german.overview || (await page.locator("#admin-tab-spaces").textContent())?.trim() !== german.spaces || (await page.locator("#admin-space-create-open").textContent())?.trim() !== german.newSpace) throw new Error("admin live journey German navigation mismatch");
    const spaces = await request("GET", "/admin/api/spaces?limit=100");
    if (spaces.status !== 200 || !Array.isArray(spaces.value?.rows) || !spaces.value.rows.some((space: any) => space.name === fixture.mutation.name)) throw new Error("admin live journey bounded created-space read mismatch");
    console.log(`admin-live-journey: SQLite overview/create/read, EN/DE UI, operation poll, and ${terminal.state === "cancelled" ? "cancel" : "already-terminal cancel race"} passed`);
  } catch (error) {
    const diagnostics = run?.output().slice(-4_096) ?? "";
    const browserOutput = browserDiagnostics.join("\n");
    throw new Error(`${error instanceof Error ? error.message : "admin live journey failed"}${browserOutput ? `\nbrowser diagnostics:\n${browserOutput}` : ""}${diagnostics ? `\nhub diagnostics:\n${diagnostics}` : ""}`);
  } finally {
    await browser?.close().catch(() => undefined);
    await relay?.stop().catch(() => undefined);
    if (run) await finishLocalHub(run);
  }
}

/** 🛡️ `os-hub-admin`'s build MUST land before cargo ever runs `main()` for real — `HubState.
 * admin_dir` (§C0 `OS_HUB_ADMIN_DIR`, else the compile-time default) is read at hub STARTUP, not
 * build time, so this is a runtime prerequisite, not a Cargo `build.rs` concern. Zero-touch/
 * cross-platform: `bun nx run os-hub-admin:build` is the same command every OS/devcontainer already
 * runs for every other nx target here. */
function buildAdminSpa(repoRoot: string): void {
  runCmd("bun", ["nx", "run", "os-hub-admin:build"], { cwd: repoRoot, ...orchestratorBudgetOpts() });
}

class SetupScript extends BundleScript {
  run(): void {
    runCargo(["fetch", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class BuildScript extends BundleScript {
  run(): void {
    buildAdminSpa(this.repoRoot);
    runCargo(["build", "--release", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    // 🎛️ `--all-features` so a plain `bun ./📜️script.ts test` covers the full old 5-crate baseline
    // (directory core + sqlite/postgres/neo4j backends + the bin's own WS/REST suite) in one run —
    // `postgres`'s own tests still need a live Docker daemon regardless of this flag (pre-existing,
    // not a regression from the merge).
    runCargoTestBudgeted(["semio-hub"], this.repoRoot, ["--all-features", ...rest]);
  }
}

class ArtifactCasCheckScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--lib", "artifact_chunk_cas", ...segments], this.root, { ...process.env, RUST_MIN_STACK: "16777216" });
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", "artifact_cas_maintenance", ...segments], this.root, { ...process.env, RUST_MIN_STACK: "16777216" });
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root);
  }
}

async function proveScopedDirectorySocketRevocationFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "📇️directory", "🧪️fixtures", "🔌️scoped-socket-revocation-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid scoped directory socket fixture: ${JSON.stringify(validate.errors)}`);
  const sameScope = (left: any, right: any): boolean => left.spaceId === right.spaceId && left.documentId === right.documentId;
  const scopedClasses = new Set(["document-announced", "checkpoint", "retention", "rebootstrap", "presence", "connection"]);
  const decide = (vector: any): { outcome: string; closeCode: number | null; cursorAdvance: boolean; textFrames: number } => {
    if (!sameScope(vector.grantScope, vector.urlScope)) return { outcome: "deny-before-upgrade", closeCode: 4401, cursorAdvance: false, textFrames: 0 };
    if (vector.gateWinner === "removal" || vector.binding === "unauthorized" || !vector.descriptor || !vector.live) return { outcome: "close-unauthorized", closeCode: 4401, cursorAdvance: false, textFrames: 0 };
    if (vector.binding === "unavailable") return { outcome: "close-unavailable", closeCode: 1013, cursorAdvance: false, textFrames: 0 };
    if (!scopedClasses.has(vector.message.class) || vector.message.scope === null || !sameScope(vector.grantScope, vector.message.scope)) return { outcome: "skip-unrelated", closeCode: null, cursorAdvance: false, textFrames: 0 };
    return { outcome: "deliver", closeCode: null, cursorAdvance: ["document-announced", "checkpoint", "retention"].includes(vector.message.class), textFrames: 1 };
  };
  for (const vector of fixture.vectors) {
    const actual = decide(vector);
    const expected = { outcome: vector.expected, closeCode: vector.closeCode, cursorAdvance: vector.cursorAdvance, textFrames: vector.textFrames };
    if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`scoped directory decision differs for ${vector.name}: ${JSON.stringify(actual)}`);
  }
  const hostile = [
    { ...fixture, extra: true },
    { ...fixture, scope: { ...fixture.scope, spaceId: "x".repeat(129) } },
    { ...fixture, clientCloses: [...fixture.clientCloses, fixture.clientCloses[0]] },
  ];
  if (hostile.some(candidate => validate(candidate))) throw new Error("scoped directory schema admitted a hostile boundary mutation");
  for (const close of fixture.clientCloses) {
    const terminal = close.code === 4401;
    if (terminal !== close.terminal || close.reconnect === terminal) throw new Error(`scoped directory client close mismatch for ${close.code}`);
  }
  const relayPath = "/directory/spaces/space%2Fa/documents/document%20b/socket-grants";
  if (localRelayUpstreamPath("POST", new URL(`http://relay.invalid/_semio/hub${relayPath}`)) !== relayPath) throw new Error("scoped directory relay denied the exact bounded grant path");
  if (localRelayUpstreamPath("POST", new URL(`http://relay.invalid/_semio/hub${relayPath}?extra=1`)) !== undefined) throw new Error("scoped directory relay admitted an arbitrary query");
  if (localRelayUpstreamPath("GET", new URL(`http://relay.invalid/_semio/hub${relayPath}`)) !== undefined) throw new Error("scoped directory relay admitted the wrong method");
  console.log(`scoped-directory-socket-oracle: AJV=1 decisions=${fixture.vectors.length} hostiles=${hostile.length} client-closes=${fixture.clientCloses.length} relay=3`);
}

class SocketGrantCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await proveScopedDirectorySocketRevocationFixture(this.repoRoot);
    if (segments[0] === "oracle") return;
    const tests = [
      "tests::scoped_directory_socket_ledger_indexes_and_invalidates_exact_membership",
      "tests::scoped_directory_socket_message_matching_is_body_exact_and_removal_private",
      "tests::scoped_directory_socket_admin_removal_uses_the_same_membership_fence",
      "tests::scoped_directory_socket_route_rejects_scope_substitution_and_rest_removal_closes_without_event",
      "tests::scoped_directory_socket_removal_and_delivery_have_one_total_membership_order",
      "tests::directory_socket_forced_lag_is_scope_authorized_and_closes_1013",
      "tests::document_socket_forced_lag_sends_verified_control_then_closes_1013",
      "tests::socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke",
      "tests::socket_directory_revoke_after_admission_suppresses_replay_without_deadlock",
      "tests::socket_directory_visibility_requires_membership_even_for_public_spaces",
      "tests::socket_grant_directory_route_uses_credential_free_hello_and_revokes_live",
      "tests::socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live",
      "tests::socket_grant_ledger_is_bounded_single_consume_restart_scoped_and_revoke_race_safe",
      "tests::socket_grant_revoke_and_welcome_have_a_bounded_binding_linearization",
      "tests::socket_grant_revoke_before_broadcast_authorization_suppresses_frame",
      "tests::socket_grant_revoke_before_command_admission_has_no_storage_effect",
      "tests::socket_grant_revoke_before_lag_authorization_reads_no_private_control",
    ];
    const env = { ...process.env, RUST_MIN_STACK: "268435456" };
    for (const test of tests) runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", test, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--lib", "typed_capabilities_match_neutral_sha256_vectors_and_fixed_boundaries", "--", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--lib", "socket_binding_reads_are_exact_id_generation_selector_scope_and_status", "--", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-framework-replication", "client_frame_socket_hello_v1_round_trips_without_credentials", "--", "--test-threads=1"], this.root, env);
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root, env);
  }
}

class ScopedDirectorySocketCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "all";
    if (segments.length > 1 || !["all", "source", "native", "process"].includes(phase)) throw new Error("scoped-directory-socket-check accepts source, native, or process");
    await proveScopedDirectorySocketRevocationFixture(this.repoRoot);
    if (phase === "all" || phase === "source") {
      runCmd("bun", [join(this.repoRoot, "📜️script.ts"), "nx", "run", "@semio-tech/framework-os:test-quick", "--skip-nx-cache", "--", "--run", "-t", "round trips scoped directory worker ownership without flattening scope|binds one document scope and treats close 4401 as terminal without reacquiring|backbone worker owns one full scoped stream and retires it terminally on 4401"], {
        cwd: this.repoRoot,
        ...orchestratorBudgetOpts(),
      });
      console.log("scoped-directory-socket-source-check: neutral=19 hostile=3 browser-terminal=3");
    }
    if (phase === "all" || phase === "native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        groups: [{
          package: "semio-framework-os-kernel",
          target: { kind: "lib", name: "semio_framework_os_kernel" },
          laws: [
            "scoped_stream_close_4401_is_terminal_and_never_redials",
            "scoped_stream_issues_and_dials_the_same_encoded_scope",
          ],
        }],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        listBudgetMs: 60_000,
        lawBudgetMs: 60_000,
        progress(event) { console.log(`scoped-directory-socket-native ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`scoped-directory-socket-native-receipt: ${JSON.stringify(receipt)}`);
    }
    if (phase === "all" || phase === "process") {
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{
          package: "semio-hub",
          target: { kind: "bin", name: "os-hub" },
          cargoArgs: ["--all-features"],
          laws: [
            "scoped_directory_socket_ledger_indexes_and_invalidates_exact_membership",
            "scoped_directory_socket_message_matching_is_body_exact_and_removal_private",
            "scoped_directory_socket_route_rejects_scope_substitution_and_rest_removal_closes_without_event",
            "scoped_directory_socket_admin_removal_uses_the_same_membership_fence",
            "scoped_directory_socket_removal_and_delivery_have_one_total_membership_order",
          ],
        }],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        listBudgetMs: 60_000,
        lawBudgetMs: 60_000,
        progress(event) { console.log(`scoped-directory-socket-process ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`scoped-directory-socket-process-receipt: ${JSON.stringify(receipt)}`);
      runCmd("cargo", ["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
    }
  }
}

type ExecutionTargetRelayFixture = {
  readonly intent: Record<string, unknown>;
  readonly limits: { readonly requestBytes: number; readonly manifestBytes: number; readonly componentBytes: number; readonly descriptorBytes: number; readonly inFlight: number; readonly hubDeadlineMs: number; readonly relayDeadlineMs: number };
  readonly routes: readonly { readonly id: string; readonly method: string; readonly path: string; readonly admitted: boolean }[];
  readonly responses: readonly { readonly id: string; readonly asset: string; readonly bytes: number; readonly delayMs: number; readonly status: number }[];
  readonly fences: readonly { readonly mutation: string; readonly expected: string }[];
};

type NativeArtifactProviderFrontierFixture = {
  readonly schema: "semio.hub.native-artifact-provider-frontier/v1";
  readonly headless: { readonly defaultFeatures: false; readonly features: readonly ["sqlite"]; readonly directPluginDependencies: readonly [] };
  readonly production: {
    readonly feature: "native-artifact-execution";
    readonly providerId: "stdio+gis/native-codecs/v1";
    readonly receiptCount: 28;
    readonly pluginDependencies: readonly ["stdio/full-artifact-catalog", "gis"];
  };
  readonly configuredWithoutProvider: "reject";
};

async function proveNativeArtifactProviderFrontier(repoRoot: string): Promise<number> {
  const fixtureRoot = join(repoRoot, "🌎️hub/🧪️fixtures/🧭️native-artifact-provider-frontier-v1");
  const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as NativeArtifactProviderFrontierFixture;
  const { default: Ajv2020 } = await import("ajv/dist/2020.js");
  const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
  if (!validate(fixture)) throw new Error("native artifact provider frontier fixture: " + JSON.stringify(validate.errors));
  const manifest = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/Cargo.toml"), "utf8");
  const feature = fixture.production.feature.replace(/[.*+?^$()|[\]\\]/gu, "\\$&");
  if (!new RegExp("^default\\s*=\\s*\\[\"sqlite\",\\s*\"" + feature + "\"\\]$", "mu").test(manifest)) throw new Error("production Hub default does not retain native artifact execution");
  const featureRow = manifest.match(new RegExp("^" + feature + "\\s*=\\s*\\[([^\\n]+)\\]$", "mu"))?.[1] ?? "";
  for (const dependency of ['"dep:semio-s-plugin-stdio"', '"semio-s-plugin-stdio/full-artifact-catalog"', '"dep:semio-s-plugin-gis"']) {
    if (!featureRow.includes(dependency)) throw new Error("native artifact execution feature omitted " + dependency);
  }
  for (const dependency of ["semio-s-plugin-stdio", "semio-s-plugin-gis"]) {
    const row = manifest.match(new RegExp("^" + dependency + "\\s*=\\s*\\{([^\\n]+)\\}$", "mu"))?.[1] ?? "";
    if (!row.includes("optional = true") || !row.includes("default-features = false")) throw new Error(dependency + " is not an optional no-default dependency");
  }
  const authorityRoot = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/🦀️.rs"), "utf8");
  if (!/#\[cfg\(feature = "native-artifact-execution"\)\]\s+#\[path = "📇️native-openable-provider\/🦀️\.rs"\]\s+pub mod native_openable_provider;/u.test(authorityRoot)) throw new Error("native provider module escaped its production feature");
  const trusted = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"), "utf8");
  if (!trusted.includes("pub trait NativeCodecProviderSourceV1: Sync") || trusted.includes("use super::native_openable_provider::NativeCodecProviderSetV1;")) throw new Error("trusted catalog core still owns a concrete native plugin provider");
  const provider = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs"), "utf8");
  if (!provider.includes('pub const NATIVE_OPENABLE_PROVIDER_SET_V1_ID: &str = "' + fixture.production.providerId + '";') || !provider.includes("pub const NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = " + fixture.production.receiptCount + ";")) throw new Error("production provider identity or receipt closure drifted");
  const startup = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");
  if (!startup.includes("providers: Option<&dyn NativeCodecProviderSourceV1>") || !startup.includes("configured trusted catalog requires the native-artifact-execution provider") || !startup.includes("configured_catalog_without_a_native_provider_fails_closed")) throw new Error("headless configured-catalog startup no longer fails closed");
  console.log("hub-native-artifact-provider-frontier-oracle: AJV=1 headless=" + fixture.headless.features.join("+") + " plugin-deps=" + fixture.headless.directPluginDependencies.length + " production-receipts=" + fixture.production.receiptCount + " configured-no-provider=" + fixture.configuredWithoutProvider);
  return 12;
}

async function proveExecutionTargetRelay(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/🧪️fixtures/🪪️execution-target-relay-v1");
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as ExecutionTargetRelayFixture;
  const ajv = new Ajv({ allErrors: true, strict: true });
  const validate = ajv.compile(schema);
  if (!validate(fixture)) throw new Error(`execution-target relay fixture: ${JSON.stringify(validate.errors)}`);
  const limits = fixture.limits;
  if (limits.requestBytes !== EXECUTION_TARGET_RELAY_REQUEST_MAX_BYTES || limits.manifestBytes !== EXECUTION_TARGET_RELAY_MANIFEST_MAX_BYTES || limits.componentBytes !== DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES || limits.descriptorBytes !== DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES || limits.inFlight !== EXECUTION_TARGET_RELAY_MAX_IN_FLIGHT || limits.relayDeadlineMs !== EXECUTION_TARGET_RELAY_DEADLINE_MS) throw new Error("execution-target relay bound drift");
  const hub = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");
  const selection = hub.slice(hub.indexOf("async fn document_execution_target_selection("), hub.indexOf("async fn issue_document_execution_target("));
  const finalFence = selection.slice(selection.indexOf("fields.validate()"));
  if (!hub.includes(`const DOCUMENT_EXECUTION_TARGET_DEADLINE_MS: u64 = ${limits.hubDeadlineMs.toLocaleString("en-US").replaceAll(",", "_")};`) || !finalFence.includes("subject.revalidate(") || !finalFence.includes("state.directory.head_seq().await") || !finalFence.includes("DocumentOpenPlanErrorCodeV1::Stale") || !hub.includes('fixture["fences"]')) throw new Error("execution-target final authorization/revision fence or native corpus missing");
  const admittedRoute = ajv.getSchema(`${schema.$id}#/definitions/admittedRoute`)!;
  for (const row of fixture.routes) {
    if (admittedRoute(row) !== row.admitted) throw new Error(`execution-target independent schema route mismatch: ${row.id}`);
    if ((localRelayUpstreamPath(row.method, new URL(row.path, "http://127.0.0.1")) !== undefined) !== row.admitted) throw new Error(`execution-target production route mismatch: ${row.id}`);
  }
  const slowAbort = new AbortController();
  const slowBody = new Request("http://127.0.0.1", { method: "POST", body: new ReadableStream<Uint8Array>() });
  const slowRead = readLocalRelayBody(slowBody, limits.requestBytes, slowAbort.signal).then(() => false, () => true);
  slowAbort.abort();
  if (!(await Promise.race([slowRead, Bun.sleep(100).then(() => false)]))) throw new Error("execution-target slow upload did not release on cancellation");
  const uiOrigin = "http://127.0.0.1:6066";
  const intentBody = JSON.stringify(fixture.intent);
  parseDocumentOpenIntentV1(fixture.intent);
  let effects = 0;
  let responseBytes = 1;
  let delayMs = 0;
  let release: Promise<void> | undefined;
  const upstream = Bun.serve({
    hostname: "127.0.0.1", port: 0,
    async fetch(request): Promise<Response> {
      effects += 1;
      if (request.headers.get("authorization") !== `Bearer ${browserBrokerOracleEnvelope().capability}`) throw new Error("execution-target upstream credential mismatch");
      const body = await request.text();
      if (body !== intentBody) throw new Error("execution-target intent body changed in transit");
      if (release) await release;
      if (delayMs) await Bun.sleep(delayMs);
      return new Response(new Uint8Array(responseBytes).fill(73), { headers: { "content-type": "application/octet-stream", "cache-control": "no-store" } });
    },
  });
  let proof = randomBytes(32);
  const relay = startLocalBrowserRelay(`http://127.0.0.1:${upstream.port}`, uiOrigin, browserBrokerOracleEnvelope(), Buffer.from(proof));
  const request = async (path: string, method = "POST", body = intentBody, signal?: AbortSignal): Promise<Response> => {
    const current = proof;
    proof = randomBytes(32);
    const response = await fetch(`${relay.url}${path}`, {
      method, body: method === "GET" ? undefined : body, signal, redirect: "error",
      headers: { host: new URL(uiOrigin).host, origin: uiOrigin, referer: `${uiOrigin}/`, "sec-fetch-site": "same-origin", "x-semio-local-relay": relay.secret.toString("hex"), "x-semio-browser-broker": current.toString("hex"), "x-semio-browser-broker-next": browserBrokerProofDigest(proof).toString("hex"), "content-type": "application/json" },
    });
    if (response.headers.get("x-semio-browser-broker-advanced") !== "1") proof = current;
    return response;
  };
  const path = (asset: string): string => `/_semio/hub/spaces/studio/documents/map/execution-target/${asset}`;
  try {
    for (const row of fixture.routes) {
      const before = effects;
      const response = await request(row.path, row.method);
      if (response.status !== (row.admitted ? 200 : 404) || effects !== before + Number(row.admitted)) throw new Error(`execution-target live route mismatch: ${row.id}`);
      await response.arrayBuffer();
    }
    for (const row of fixture.responses) {
      responseBytes = row.bytes;
      delayMs = row.delayMs;
      const response = await request(path(row.asset));
      if (response.status !== row.status) throw new Error(`execution-target live response mismatch: ${row.id}: ${response.status}`);
      const bytes = new Uint8Array(await response.arrayBuffer());
      if (row.status === 200 && (response.headers.get("content-length") !== String(row.bytes) || response.headers.get("cache-control") !== "no-store" || bytes.length !== row.bytes || bytes.some((value) => value !== 73))) throw new Error(`execution-target live bytes mismatch: ${row.id}`);
    }
    const before = effects;
    const oversized = await request(path("manifest"), "POST", "x".repeat(fixture.limits.requestBytes + 1));
    if (oversized.status !== 413 || effects !== before || oversized.headers.has("x-semio-browser-broker-advanced")) throw new Error("execution-target request overflow crossed ratchet or upstream");
    delayMs = 0;
    responseBytes = 1024 * 1024 + 17;
    let resolveRelease!: () => void;
    release = new Promise<void>((resolve) => { resolveRelease = resolve; });
    const pending: Promise<Response>[] = [];
    try {
      for (let index = 0; index < fixture.limits.inFlight; index += 1) {
        pending.push(request(path("component")));
        const deadline = Date.now() + 2_000;
        while (effects < before + index + 1 && Date.now() < deadline) await Bun.sleep(5);
        if (effects !== before + index + 1) throw new Error("execution-target concurrent admission did not reach upstream");
      }
      const saturated = await request(path("component"));
      if (saturated.status !== 503 || saturated.headers.has("x-semio-browser-broker-advanced") || effects !== before + fixture.limits.inFlight) throw new Error("execution-target saturation consumed authority or exceeded capacity");
    } finally {
      resolveRelease();
      await Promise.all(pending.map(async (response) => { if (!(await response).ok) throw new Error("execution-target admitted concurrent request failed"); }));
      release = undefined;
    }
    const recovered = await request(path("descriptor"));
    if (!recovered.ok) throw new Error("execution-target capacity did not recover");
    await recovered.arrayBuffer();
    const cancelBefore = effects;
    let resolveCancelled!: () => void;
    release = new Promise<void>((resolve) => { resolveCancelled = resolve; });
    const abort = new AbortController();
    const cancelled = request(path("component"), "POST", intentBody, abort.signal).then(() => { throw new Error("execution-target cancelled request returned bytes"); }, () => undefined);
    try {
      const deadline = Date.now() + 2_000;
      while (effects < cancelBefore + 1 && Date.now() < deadline) await Bun.sleep(5);
      if (effects !== cancelBefore + 1) throw new Error("execution-target cancellation did not reach upstream");
      abort.abort();
      await cancelled;
    } finally {
      resolveCancelled();
      release = undefined;
    }
    responseBytes = 19;
    const afterCancel = await request(path("component"));
    if (!afterCancel.ok || (await afterCancel.arrayBuffer()).byteLength !== 19) throw new Error("execution-target cancellation did not release capacity and ratchet");
    console.log(`execution-target relay runtime routes=${fixture.routes.length} responses=${fixture.responses.length} bounded-capacity=${fixture.limits.inFlight} exact-bytes=true`);
    return fixture.routes.length + fixture.responses.length + fixture.fences.length + 5;
  } finally {
    proof.fill(0);
    await relay.stop();
    await upstream.stop(true);
  }
}

class ExecutionTargetRelayCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("execution-target-relay-check accepts only --native");
    console.log("execution-target-provider-frontier: checks=" + await proveNativeArtifactProviderFrontier(this.repoRoot));
    console.log(`execution-target-relay-check: checks=${await proveExecutionTargetRelay(this.repoRoot)}`);
    await proveBrowserBrokerRelay();
    console.log("execution-target-relay-check: existing browser proof-ratchet runtime regression clean");
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-hub", target: { kind: "bin", name: "os-hub" }, cargoArgs: ["--no-default-features", "--features", "sqlite"], laws: ["configured_catalog_without_a_native_provider_fails_closed", "execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body", "execution_target_selection_final_fence_matches_neutral_races"] }],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        progress(event) { console.log(`execution-target-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      console.log(`execution-target-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}

class BrowserBrokerCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveBrowserBrokerRelay();
    runCmd("bun", ["nx", "run", "@semio-tech/framework-os:test-quick", "--skip-nx-cache", "--", "--run", "-t", "browser broker proof ratchet|queues a directory command while the hub is unreachable"], {
      cwd: this.repoRoot,
      ...orchestratorBudgetOpts(),
    });
    console.log("browser-broker-check: raw-local and shard denial, one-use ratchet, replay rejection, rotated TTL, upstream 401 epoch closure, cancel-after-send, and redaction passed");
  }
}

class CanonicalPairCheckScript extends BundleScript {
  run(): void {
    const env = { ...process.env, RUST_MIN_STACK: "268435456" };
    runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "canonical_pair", "--", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--bin", "os-hub", "canonical_pair_route", "--", "--test-threads=1"], this.root, env);
    runCmd("bun", ["nx", "run", "os-hub-ts:test-quick", "--", "--run", "-t", "canonical checkpoint pair neutral contract"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root, env);
  }
}

function documentOpenLengthPrefix(bytes: Uint8Array): Buffer {
  const length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(bytes.byteLength));
  return Buffer.concat([length, bytes]);
}

function documentOpenDescriptorEncoding(descriptor: Record<string, any>): Buffer {
  const text = (value: unknown): Buffer => {
    if (typeof value !== "string" || value.length === 0) throw new Error("document-open oracle invalid descriptor text");
    return Buffer.from(value, "utf8");
  };
  const hash = (value: unknown): Buffer => {
    if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value) || /^0{64}$/u.test(value)) throw new Error("document-open oracle invalid descriptor hash");
    return Buffer.from(value, "hex");
  };
  const integer = (value: unknown, width: 4 | 8): Buffer => {
    if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new Error("document-open oracle invalid descriptor integer");
    const output = Buffer.alloc(width);
    if (width === 4) output.writeUInt32BE(value);
    else output.writeBigUInt64BE(BigInt(value));
    return output;
  };
  const fields = [
    text(descriptor.spaceId),
    text(descriptor.documentId),
    text(descriptor.artifactKind),
    text(descriptor.artifactSchema),
    text(descriptor.owner?.pluginId),
    text(descriptor.owner?.packageId),
    text(descriptor.owner?.version),
    hash(descriptor.owner?.packageHash),
    hash(descriptor.packSchemaHash),
    integer(descriptor.bootstrapVersion, 4),
    integer(descriptor.bootstrapFrontier?.headSeq, 8),
    integer(descriptor.bootstrapFrontier?.commitSeq, 8),
    integer(descriptor.bootstrapFrontier?.epoch, 8),
    hash(descriptor.bootstrapSnapshotHash),
  ];
  return Buffer.concat([Buffer.from("semio.document-descriptor.digest.v1\0"), ...fields.map(documentOpenLengthPrefix)]);
}

function documentOpenCatalogEncoding(rows: readonly Record<string, any>[]): Buffer {
  if (rows.length === 0 || rows.length > 1_024) throw new Error("document-open oracle invalid catalog size");
  const rowCount = Buffer.alloc(4);
  rowCount.writeUInt32BE(rows.length);
  const encodedRows = rows.map((row) => {
    documentOpenNeutralParentDialect(row);
    const fields = [
      Buffer.from(row.package.pluginId, "utf8"),
      Buffer.from(row.package.packageId, "utf8"),
      Buffer.from(row.package.version, "utf8"),
      Buffer.from(row.package.componentSha256, "hex"),
      Buffer.from(row.package.componentBlake3, "hex"),
      Buffer.from(row.package.descriptorByteSha256, "hex"),
      Buffer.from(row.artifact.kind, "utf8"),
      Buffer.from(row.artifact.schema, "utf8"),
      Buffer.from(row.artifact.packSchemaHash, "hex"),
      Buffer.from(row.parentDialect.artifactKind, "utf8"),
      Buffer.from(row.parentDialect.standard, "utf8"),
      Buffer.from(row.parentDialect.subset, "utf8"),
      Buffer.from(row.surface.surfaceId, "utf8"),
      Buffer.from(row.surface.appId, "utf8"),
      Buffer.from(row.surface.windowKindId, "utf8"),
      Buffer.from(row.surface.role, "utf8"),
      Buffer.from(row.surface.rendererTarget, "utf8"),
      Buffer.from([row.grant.read ? 1 : 0, row.grant.write ? 1 : 0, row.grant.observe ? 1 : 0]),
    ];
    return Buffer.concat(fields.map(documentOpenLengthPrefix));
  });
  return Buffer.concat([Buffer.from("semio/hub/openable-document-catalog/v1\0"), rowCount, ...encodedRows]);
}

function documentOpenMutation(value: Record<string, any>, path: string, replacement: unknown): Record<string, any> {
  const mutated = structuredClone(value);
  const components = path.split(".");
  let target = mutated;
  for (const component of components.slice(0, -1)) {
    if (target[component] === null || typeof target[component] !== "object") throw new Error(`document-open oracle invalid mutation path ${path}`);
    target = target[component];
  }
  target[components.at(-1)!] = replacement;
  return mutated;
}

function documentOpenRemoval(value: Record<string, any>, path: string): Record<string, any> {
  const mutated = structuredClone(value);
  const components = path.split(".");
  let target = mutated;
  for (const component of components.slice(0, -1)) {
    if (target[component] === null || typeof target[component] !== "object") throw new Error(`document-open oracle invalid removal path ${path}`);
    target = target[component];
  }
  delete target[components.at(-1)!];
  return mutated;
}

function documentOpenNeutralObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, any> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("object");
  const object = value as Record<string, any>;
  const keys = Object.keys(object);
  if (required.some((key) => !(key in object)) || keys.some((key) => !required.includes(key) && !optional.includes(key))) throw new Error("keys");
  return object;
}

function documentOpenNeutralText(value: unknown, maximum = 256): string {
  if (typeof value !== "string" || value.length === 0 || Buffer.byteLength(value, "utf8") > maximum || /\p{Cc}/u.test(value)) throw new Error("text");
  return value;
}

function documentOpenNeutralDialect(value: unknown, artifactKind: unknown): Record<string, string> {
  const dialect = documentOpenNeutralObject(value, ["artifactKind", "standard", "subset"]);
  for (const value of Object.values(dialect)) {
    const text = documentOpenNeutralText(value);
    if (text.trim() !== text) throw new Error("dialect-text");
  }
  if (dialect.artifactKind !== artifactKind) throw new Error("dialect-kind");
  return dialect as Record<string, string>;
}

function documentOpenNeutralParentDialect(value: unknown): Record<string, string> {
  const row = documentOpenNeutralObject(value, ["package", "artifact", "parentDialect", "surface", "grant"]);
  const artifact = documentOpenNeutralObject(row.artifact, ["kind", "schema", "packSchemaHash"]);
  return documentOpenNeutralDialect(row.parentDialect, artifact.kind);
}

function documentOpenNeutralReceipt(value: unknown): string {
  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
  if (typeof value !== "string" || !/^open\.v1\.[A-Za-z0-9_-]{43}$/u.test(value) || (alphabet.indexOf(value.at(-1)!) & 0b11) !== 0) throw new Error("receipt");
  return value;
}

function documentOpenNeutralIntent(candidate: unknown): void {
  const root = documentOpenNeutralObject(candidate, ["schema", "version", "scope", "clientInstanceId"], ["requestedSurfaceId"]);
  if (root.schema !== "semio.hub.document-open-intent/v1" || root.version !== 1) throw new Error("intent-version");
  const scope = documentOpenNeutralObject(root.scope, ["spaceId", "documentId"]);
  documentOpenNeutralText(scope.spaceId);
  documentOpenNeutralText(scope.documentId);
  documentOpenNeutralText(root.clientInstanceId, 128);
  if (root.requestedSurfaceId !== undefined) documentOpenNeutralText(root.requestedSurfaceId);
}

function documentOpenNeutralExchange(candidate: unknown): void {
  const root = documentOpenNeutralObject(candidate, ["schema", "version", "planReceipt"]);
  if (root.schema !== "semio.hub.document-plan-socket-grant-intent/v1" || root.version !== 1) throw new Error("exchange-version");
  documentOpenNeutralReceipt(root.planReceipt);
}

function documentOpenNeutralIssueOutcome(candidate: Record<string, any>, subjectKind: unknown, role: unknown, fixture: Record<string, any>): { code: string; surfaceId?: string; write?: boolean } {
  try {
    documentOpenNeutralIntent(candidate);
  } catch {
    return { code: "denied" };
  }
  if (candidate.scope.spaceId !== fixture.descriptor.spaceId || candidate.scope.documentId !== fixture.descriptor.documentId) return { code: "denied" };
  if (subjectKind !== "share" && (subjectKind !== "session" || !new Set(["author", "spectator"]).has(role))) return { code: "denied" };
  const writable = subjectKind === "session" && role === "author";
  const matches = fixture.catalogRows.filter((row: Record<string, any>) => {
    try {
      documentOpenNeutralParentDialect(row);
      const packageValue = documentOpenNeutralObject(row.package, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256"]);
      const artifact = documentOpenNeutralObject(row.artifact, ["kind", "schema", "packSchemaHash"]);
      const surface = documentOpenNeutralObject(row.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
      const grant = documentOpenNeutralObject(row.grant, ["read", "write", "observe"]);
      for (const text of [packageValue.pluginId, packageValue.packageId, packageValue.version, artifact.kind, artifact.schema, surface.surfaceId, surface.appId, surface.windowKindId]) documentOpenNeutralText(text);
      for (const digest of [packageValue.componentSha256, packageValue.componentBlake3, packageValue.descriptorByteSha256, artifact.packSchemaHash]) {
        if (typeof digest !== "string" || !/^[0-9a-f]{64}$/u.test(digest) || /^0{64}$/u.test(digest)) throw new Error("digest");
      }
      if (!new Set(["viewer", "editor"]).has(surface.role) || !new Set(["react", "wgpu", "wasm"]).has(surface.rendererTarget)) throw new Error("surface");
      if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean" || grant.write !== (surface.role === "editor")) throw new Error("grant");
      return (
        packageValue.pluginId === fixture.descriptor.owner.pluginId &&
        packageValue.packageId === fixture.descriptor.owner.packageId &&
        packageValue.version === fixture.descriptor.owner.version &&
        packageValue.componentSha256 === fixture.descriptor.owner.packageHash &&
        artifact.kind === fixture.descriptor.artifactKind &&
        artifact.schema === fixture.descriptor.artifactSchema &&
        artifact.packSchemaHash === fixture.descriptor.packSchemaHash &&
        grant.write === writable &&
        (candidate.requestedSurfaceId === undefined || candidate.requestedSurfaceId === surface.surfaceId)
      );
    } catch {
      return false;
    }
  });
  if (matches.length !== 1) return { code: "component-unavailable" };
  return { code: "accepted", surfaceId: matches[0].surface.surfaceId, write: matches[0].grant.write };
}

function documentOpenNeutralStructure(candidate: Record<string, any>, nowMs: number): void {
  const hash = (value: unknown): string => {
    if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value) || /^0{64}$/u.test(value)) throw new Error("hash");
    return value;
  };
  const integer = (value: unknown, positive = false): number => {
    if (typeof value !== "number" || !Number.isSafeInteger(value) || value < (positive ? 1 : 0)) throw new Error("integer");
    return value;
  };
  const root = documentOpenNeutralObject(candidate, ["schema", "version", "receipt", "expiresAtUnixMs", "scope", "descriptorDigestV1", "catalog", "package", "artifact", "parentDialect", "surface", "grant", "revalidation"], ["checkpoint"]);
  if (root.schema !== "semio.hub.document-open-plan/v1" || root.version !== 1) throw new Error("version");
  documentOpenNeutralReceipt(root.receipt);
  const expiry = integer(root.expiresAtUnixMs, true);
  if (expiry <= nowMs) throw new Error("expired");
  if (expiry - nowMs > 30_000) throw new Error("ttl");
  const scope = documentOpenNeutralObject(root.scope, ["spaceId", "documentId"]);
  documentOpenNeutralText(scope.spaceId);
  documentOpenNeutralText(scope.documentId);
  hash(root.descriptorDigestV1);
  const catalog = documentOpenNeutralObject(root.catalog, ["generationId"]);
  hash(catalog.generationId);
  const packageValue = documentOpenNeutralObject(root.package, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256"]);
  documentOpenNeutralText(packageValue.pluginId);
  documentOpenNeutralText(packageValue.packageId);
  documentOpenNeutralText(packageValue.version);
  hash(packageValue.componentSha256);
  hash(packageValue.componentBlake3);
  hash(packageValue.descriptorByteSha256);
  const artifact = documentOpenNeutralObject(root.artifact, ["kind", "schema", "packSchemaHash"]);
  documentOpenNeutralText(artifact.kind);
  documentOpenNeutralText(artifact.schema);
  hash(artifact.packSchemaHash);
  documentOpenNeutralDialect(root.parentDialect, artifact.kind);
  const surface = documentOpenNeutralObject(root.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
  documentOpenNeutralText(surface.surfaceId);
  documentOpenNeutralText(surface.appId);
  documentOpenNeutralText(surface.windowKindId);
  if (!new Set(["viewer", "editor"]).has(surface.role) || !new Set(["react", "wgpu", "wasm"]).has(surface.rendererTarget)) throw new Error("surface");
  const grant = documentOpenNeutralObject(root.grant, ["read", "write", "observe"]);
  if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean" || grant.write !== (surface.role === "editor")) throw new Error("grant");
  const revalidation = documentOpenNeutralObject(root.revalidation, ["directoryRevision", "membershipGeneration"], ["sessionGeneration", "shareGeneration"]);
  integer(revalidation.directoryRevision, true);
  integer(revalidation.membershipGeneration, true);
  if ((revalidation.sessionGeneration === undefined) === (revalidation.shareGeneration === undefined)) throw new Error("binding");
  if (revalidation.sessionGeneration !== undefined) integer(revalidation.sessionGeneration, true);
  if (revalidation.shareGeneration !== undefined) integer(revalidation.shareGeneration, true);
  if (root.checkpoint !== undefined) {
    const checkpoint = documentOpenNeutralObject(root.checkpoint, ["checkpointId", "descriptorDigestV1", "baselineFrontier", "aggregateSha256"]);
    hash(checkpoint.checkpointId);
    hash(checkpoint.descriptorDigestV1);
    hash(checkpoint.aggregateSha256);
    if (checkpoint.descriptorDigestV1 !== root.descriptorDigestV1) throw new Error("checkpoint-digest");
    const frontier = documentOpenNeutralObject(checkpoint.baselineFrontier, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainHash"]);
    documentOpenNeutralText(frontier.documentId);
    documentOpenNeutralText(frontier.headEditId);
    integer(frontier.headEditOrdinal);
    integer(frontier.lastCommitSeq);
    if (
      frontier.documentId !== scope.documentId ||
      frontier.lastCommitSeq > frontier.headEditOrdinal ||
      !Array.isArray(frontier.chainHash) ||
      frontier.chainHash.length !== 32 ||
      frontier.chainHash.every((byte: unknown) => byte === 0) ||
      frontier.chainHash.some((byte: unknown) => typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255)
    )
      throw new Error("frontier");
  }
}

function documentOpenNeutralOutcome(candidate: Record<string, any>, fixture: Record<string, any>): string {
  try {
    documentOpenNeutralStructure(candidate, fixture.nowMs);
  } catch {
    return typeof candidate.expiresAtUnixMs === "number" && candidate.expiresAtUnixMs <= fixture.nowMs ? "expired" : "denied";
  }
  const selected = fixture.validPlan;
  const exact =
    candidate.scope?.spaceId === selected.scope.spaceId &&
    candidate.scope?.documentId === selected.scope.documentId &&
    candidate.descriptorDigestV1 === selected.descriptorDigestV1 &&
    candidate.catalog?.generationId === selected.catalog.generationId &&
    JSON.stringify(candidate.package) === JSON.stringify(selected.package) &&
    JSON.stringify(candidate.artifact) === JSON.stringify(selected.artifact) &&
    JSON.stringify(candidate.parentDialect) === JSON.stringify(selected.parentDialect) &&
    JSON.stringify(candidate.surface) === JSON.stringify(selected.surface) &&
    JSON.stringify(candidate.grant) === JSON.stringify(selected.grant) &&
    JSON.stringify(candidate.checkpoint) === JSON.stringify(selected.checkpoint) &&
    JSON.stringify(candidate.revalidation) === JSON.stringify(selected.revalidation);
  return exact ? "accepted" : "stale";
}

function documentOpenNeutralSocketConsumeOutcome(candidate: Record<string, any>, dialSurfaceId: unknown, fixture: Record<string, any>): string {
  try {
    documentOpenNeutralStructure(candidate, fixture.nowMs);
    documentOpenNeutralText(dialSurfaceId);
  } catch {
    return "denied";
  }
  const current = fixture.validPlan;
  const matches = fixture.catalogRows.filter(
    (row: Record<string, any>) =>
      JSON.stringify(row.package) === JSON.stringify(candidate.package) &&
      JSON.stringify(row.artifact) === JSON.stringify(candidate.artifact) &&
      JSON.stringify(row.parentDialect) === JSON.stringify(candidate.parentDialect) &&
      JSON.stringify(row.surface) === JSON.stringify(candidate.surface) &&
      JSON.stringify(row.grant) === JSON.stringify(candidate.grant),
  );
  const exact =
    candidate.scope.spaceId === fixture.descriptor.spaceId &&
    candidate.scope.documentId === fixture.descriptor.documentId &&
    candidate.descriptorDigestV1 === fixture.descriptorDigestV1 &&
    candidate.catalog.generationId === fixture.catalogEncoding.expectedGenerationId &&
    candidate.surface.surfaceId === dialSurfaceId &&
    matches.length === 1 &&
    JSON.stringify(candidate.revalidation) === JSON.stringify(current.revalidation) &&
    JSON.stringify(candidate.checkpoint) === JSON.stringify(current.checkpoint);
  return exact ? "accepted" : "denied";
}

function documentOpenPublicKeysAreRedacted(value: unknown): boolean {
  if (Array.isArray(value)) return value.every(documentOpenPublicKeysAreRedacted);
  if (value === null || typeof value !== "object") return true;
  const forbidden = new Set(["actor", "actorId", "token", "descriptor", "componentBytes", "descriptorBytes", "storageKey", "factory", "factorySymbol", "sessionId", "shareId", "catalogRows"]);
  return Object.entries(value).every(([key, child]) => !forbidden.has(key) && documentOpenPublicKeysAreRedacted(child));
}

async function proveDocumentOpenPlanFixture(repoRoot: string): Promise<void> {
  const fixturePath = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json");
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Record<string, any>;
  const hubSource = readFileSync(resolve(repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");
  const routePaths = [...hubSource.matchAll(/\.route\(\s*"([^"]+)"/g)].map((match) => match[1]!);
  const productionSource = hubSource.slice(0, hubSource.indexOf("\nmod tests {"));
  const catalogSource = readFileSync(resolve(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"), "utf8");
  if (
    !catalogSource.includes("pub parent_dialect: semio_framework::ArtifactDialect") ||
    !productionSource.includes("parent_dialect: selected.parent_dialect") ||
    !productionSource.includes("parent_dialect: DocumentOpenParentDialectV1 {") ||
    !productionSource.includes("selection.parent_dialect != authority.parent_dialect") ||
    !productionSource.includes("selected.parent_dialect != authority.parent_dialect")
  )
    throw new Error("document-open authority must retain and revalidate the verified full parent dialect");
  if (routePaths.filter((path) => path === "/spaces/{space_id}/documents/{id}/open-plan").length !== 1)
    throw new Error("document-open verified-catalog issuer route is not mounted exactly once");
  if (routePaths.filter((path) => path === "/spaces/{space_id}/documents/{id}/socket-grants").length !== 1)
    throw new Error("document-open plan exchange route is not mounted exactly once");
  if (
    !productionSource.includes("issue_document_open_plan") ||
    !productionSource.includes("DocumentOpenCatalogAuthorityV1") ||
    !productionSource.includes("resolve_document_open(&descriptor") ||
    !productionSource.includes("open_plan: open_plan_ready") ||
    !productionSource.includes("open_plan_exchange: open_plan_ready") ||
    !productionSource.includes("DocumentPlanSocketGrantIntentV1") ||
    !productionSource.includes("issue_document_plan_socket_grant") ||
    !productionSource.includes("authority_for_authenticated_exchange") ||
    !productionSource.includes("document_plan_socket_validity") ||
    !productionSource.includes("checkpoint != authority.checkpoint") ||
    productionSource.includes("post(issue_document_socket_grant)")
  )
    throw new Error("document-open catalog-gated issuer and exchange activation boundary drifted");
  const descriptorEncoding = documentOpenDescriptorEncoding(fixture.descriptor);
  const catalogEncoding = documentOpenCatalogEncoding(fixture.catalogRows);
  if (createHash("sha256").update(descriptorEncoding).digest("hex") !== fixture.descriptorDigestV1) throw new Error("document-open descriptor digest oracle mismatch");
  if (catalogEncoding.toString("hex") !== fixture.catalogEncoding.expectedHex || createHash("sha256").update(catalogEncoding).digest("hex") !== fixture.catalogEncoding.expectedGenerationId)
    throw new Error("document-open catalog generation oracle mismatch");
  const dialectFields = ["parentDialect.artifactKind", "parentDialect.standard", "parentDialect.subset"];
  if (JSON.stringify(fixture.catalogEncoding.fieldOrder.slice(9, 12)) !== JSON.stringify(dialectFields)) throw new Error("document-open parent dialect framing drifted");
  for (const field of dialectFields) {
    const rows = structuredClone(fixture.catalogRows);
    const key = field.split(".")[1]!;
    rows[0].parentDialect[key] += "-foreign";
    if (key === "artifactKind") rows[0].artifact.kind = rows[0].parentDialect[key];
    if (createHash("sha256").update(documentOpenCatalogEncoding(rows)).digest("hex") === fixture.catalogEncoding.expectedGenerationId) throw new Error(`document-open catalog lost ${field}`);
  }
  for (const mutation of fixture.parentDialectNegativeMutations) {
    const value = Object.hasOwn(mutation, "unit") ? mutation.unit.repeat(mutation.repetitions) : mutation.value;
    const row = documentOpenMutation(fixture.catalogRows[0], mutation.path, value);
    let rejected = false;
    try { documentOpenCatalogEncoding([row]); } catch { rejected = true; }
    if (!rejected) throw new Error(`document-open parent dialect admitted hostile ${mutation.path}`);
    const candidateFixture = { ...fixture, catalogRows: [row] };
    if (documentOpenNeutralIssueOutcome(fixture.intent, "session", "author", candidateFixture).code !== "component-unavailable") throw new Error(`document-open issuer admitted hostile ${mutation.path}`);
  }
  console.log(`[DEBUG] verified parent dialect source:18 fields,3 digest substitutions,${fixture.parentDialectNegativeMutations.length} hostile rows,public plan+private exchange+socket equality; native authority unverified`);
  const receiptSecret = Buffer.from(fixture.receiptDigest.receipt.slice("open.v1.".length), "base64url");
  try {
    if (receiptSecret.length !== 32 || `open.v1.${receiptSecret.toString("base64url")}` !== fixture.receiptDigest.receipt) throw new Error("document-open receipt secret grammar mismatch");
    if (createHash("sha256").update(Buffer.from(fixture.receiptDigest.domainUtf8)).update(receiptSecret).digest("hex") !== fixture.receiptDigest.expectedHex) throw new Error("document-open receipt digest oracle mismatch");
  } finally {
    receiptSecret.fill(0);
  }
  documentOpenNeutralIntent(fixture.intent);
  for (const issueCase of fixture.issueCases) {
    let candidate = structuredClone(fixture.intent);
    if (issueCase.replacePath !== undefined) candidate = documentOpenMutation(candidate, issueCase.replacePath, issueCase.value);
    if (issueCase.removePath !== undefined) candidate = documentOpenRemoval(candidate, issueCase.removePath);
    const outcome = documentOpenNeutralIssueOutcome(candidate, issueCase.subjectKind, issueCase.role, fixture);
    if (outcome.code !== issueCase.expected || outcome.surfaceId !== issueCase.expectedSurfaceId || outcome.write !== issueCase.expectedWrite)
      throw new Error(`document-open issuer vector ${issueCase.name} expected ${issueCase.expected}/${issueCase.expectedSurfaceId ?? "-"}/${issueCase.expectedWrite ?? "-"}, got ${outcome.code}/${outcome.surfaceId ?? "-"}/${outcome.write ?? "-"}`);
  }
  documentOpenNeutralStructure(fixture.validPlan, fixture.nowMs);
  documentOpenNeutralExchange(fixture.exchangeIntent);
  if (!documentOpenPublicKeysAreRedacted(fixture.validPlan)) throw new Error("document-open public plan includes a private authority field");
  for (const mutation of fixture.negativeMutations) {
    const mutated = documentOpenMutation(fixture.validPlan, mutation.path, mutation.value);
    const outcome = documentOpenNeutralOutcome(mutated, fixture);
    if (outcome !== mutation.code) throw new Error(`document-open negative vector ${mutation.name} expected ${mutation.code}, got ${outcome}`);
  }
  for (const mutation of fixture.exchangeNegativeMutations) {
    const mutated = documentOpenMutation(fixture.exchangeIntent, mutation.path, mutation.value);
    let rejected = false;
    try {
      documentOpenNeutralExchange(mutated);
    } catch {
      rejected = true;
    }
    if (!rejected) throw new Error(`document-open independent exchange codec admitted ${mutation.name}`);
  }
  for (const consumeCase of fixture.socketConsumeCases) {
    let candidate = structuredClone(fixture.validPlan);
    if (consumeCase.replacePath !== undefined) candidate = documentOpenMutation(candidate, consumeCase.replacePath, consumeCase.value);
    const outcome = documentOpenNeutralSocketConsumeOutcome(candidate, consumeCase.dialSurfaceId ?? candidate.surface.surfaceId, fixture);
    if (outcome !== consumeCase.expected) throw new Error(`document-open socket-consume vector ${consumeCase.name} expected ${consumeCase.expected}, got ${outcome}`);
  }
  console.log(`document-open-plan-oracle: descriptor=1 catalog=${fixture.catalogRows.length} receipt=1 independent-codecs=3 issuer=${fixture.issueCases.length} consume=${fixture.socketConsumeCases.length} negative=${fixture.negativeMutations.length} exchange-negative=${fixture.exchangeNegativeMutations.length} redaction=1 activation=catalog-gated-issuer+exchange passed`);

  if (JSON.stringify(parseDocumentOpenIntentV1(fixture.intent)) !== JSON.stringify(fixture.intent)) throw new Error("document-open intent codec mismatch");
  if (JSON.stringify(parseDocumentOpenPlanV1(fixture.validPlan, fixture.nowMs)) !== JSON.stringify(fixture.validPlan)) throw new Error("document-open plan codec mismatch");
  if (JSON.stringify(parseDocumentPlanSocketGrantIntentV1(fixture.exchangeIntent)) !== JSON.stringify(fixture.exchangeIntent)) throw new Error("document-open exchange codec mismatch");
  for (const mutation of fixture.negativeMutations) {
    const mutated = documentOpenMutation(fixture.validPlan, mutation.path, mutation.value);
    if (mutation.code === "denied" || mutation.code === "expired") {
      let rejected = false;
      try {
        parseDocumentOpenPlanV1(mutated, fixture.nowMs);
      } catch {
        rejected = true;
      }
      if (!rejected) throw new Error(`document-open production codec admitted ${mutation.name}`);
    }
  }
  for (const mutation of fixture.exchangeNegativeMutations) {
    const mutated = documentOpenMutation(fixture.exchangeIntent, mutation.path, mutation.value);
    let rejected = false;
    try {
      parseDocumentPlanSocketGrantIntentV1(mutated);
    } catch {
      rejected = true;
    }
    if (!rejected) throw new Error(`document-open production exchange codec admitted ${mutation.name}`);
  }
  console.log(`document-open-plan-production-parity: codecs=3 rejected=${fixture.negativeMutations.filter((mutation: Record<string, any>) => mutation.code === "denied" || mutation.code === "expired").length} exchange-rejected=${fixture.exchangeNegativeMutations.length} passed`);
}

type NativeOpenableProjectionReceipt = {
  artifact: string;
  factory_id: string;
  descriptor_codec_id: string;
  runtime_capability_id: string;
  artifact_kind: string;
  document_schema: string;
  extension: string;
  pack_schema_sha256: string;
  protocol_path: string;
};

type NativeOpenableOwnerReceipt = Omit<NativeOpenableProjectionReceipt, "protocol_path"> & { runtimeAuthorized: boolean };

/** 🧬 Proves the neutral provider projection independently from Rust codecs and loader parsers. */
async function proveNativeOpenableCatalogProviderFixture(repoRoot: string): Promise<void> {
  const fixtureRoot = join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🪪️v1");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as any;
  const fixtureSchema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8"));
  const projectionPath = join(repoRoot, fixture.providerProjection);
  const projection = JSON.parse(readFileSync(projectionPath, "utf8")) as { schema: string; provider_id: string; plugin_id: string; package_id: string; receipts: NativeOpenableProjectionReceipt[] };
  const projectionSchema = JSON.parse(readFileSync(join(projectionPath, "../🧬️native-codec-factories.schema.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ allErrors: true, strict: true });
  const claimRoot = join(repoRoot, "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️fixtures/🧾️claim-authority");
  const claimFixture = JSON.parse(readFileSync(join(claimRoot, "🔣️.json"), "utf8"));
  const validateClaims = ajv.compile(JSON.parse(readFileSync(join(claimRoot, "🧬️.schema.json"), "utf8")));
  if (!validateClaims(claimFixture)) throw new Error(`native-openable claim fixture invalid: ${JSON.stringify(validateClaims.errors)}`);
  const uniqueClaims = ajv.compile({ type: "array", items: { type: "string" }, uniqueItems: true });
  for (const row of claimFixture.cases) {
    for (const claim of row.claims) {
      if (claim.category === "codec") {
        const utf8Length = new TextEncoder().encode(claim.codecSchema).length;
        if (utf8Length !== Buffer.byteLength(claim.codecSchema, "utf8") || claim.value !== `${utf8Length}:${claim.codecSchema}:${claim.extension}`) throw new Error("native-openable codec extension framing disagrees");
      }
    }
    const pairs = row.claims.map((claim: any) => `${claim.namespace}:${claim.value}`);
    const unique = new Set(pairs).size === pairs.length;
    if (uniqueClaims(pairs) !== unique) throw new Error("native-openable AJV and independent claim uniqueness differ");
    const code = unique ? "accepted" : "artifact-definition.duplicate-claim";
    if (code !== row.code) throw new Error(`native-openable claim oracle differs for ${row.id}`);
  }
  console.log(`native-openable-claim-oracle cases=${claimFixture.cases.length}`);
  for (const [name, schema, value] of [["fixture", fixtureSchema, fixture], ["projection", projectionSchema, projection]] as const) {
    const validate = ajv.compile(schema);
    if (!validate(value)) throw new Error(`native-openable ${name} schema invalid: ${JSON.stringify(validate.errors)}`);
  }
  const definitionRoot = join(repoRoot, fixture.artifactDefinitionsRoot);
  const definitionFiles: string[] = [];
  const pending = [definitionRoot];
  while (pending.length > 0) {
    const directory = pending.pop()!;
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory()) pending.push(path);
      else if (entry.isFile() && entry.name === "📜️artifact-definition.json") definitionFiles.push(path);
    }
  }
  const owners: NativeOpenableOwnerReceipt[] = [];
  for (const path of definitionFiles.sort()) {
    const definition = JSON.parse(readFileSync(path, "utf8")) as any;
    for (const standard of definition.standards) {
      if (!/^[a-z0-9_-]+$/.test(standard.revision) || standard.id !== `${definition.id}.standard.${standard.revision}`) throw new Error("native-openable standard identity is not a canonical source-owned segment");
    }
    const claimsSeen = new Set<string>();
    for (const capability of definition.runtime_capabilities) {
      const category = capability.category;
      const prefix = category === "codec" || category === "representation" ? `${definition.standards[0].id}.${category}.` : `${definition.id}.${category}.`;
      const leaf = capability.id.startsWith(prefix) ? capability.id.slice(prefix.length) : "";
      if (!(category === "representation" ? /^[a-z0-9_-]+$/ : /^[a-z0-9_-]+\.v[1-9][0-9]*$/).test(leaf)) throw new Error("native-openable runtime capability violates canonical category ownership");
      for (const claim of capability.claims) {
        const key = `${claim.namespace}:${claim.value}`;
        if (claimsSeen.has(key)) throw new Error(`native-openable duplicate owned claim ${key}`);
        if (category === "subset-validator" && claim.namespace !== "validated-dialect") throw new Error("native-openable validator claims must be distinct from composition authority");
        claimsSeen.add(key);
      }
    }
    for (const codec of Array.isArray(definition.codecs) ? definition.codecs : []) {
      if (codec.executable_registration !== true) continue;
      const native = codec.native_factory;
      const runtime = Array.isArray(definition.runtime_capabilities)
        ? definition.runtime_capabilities.find((candidate: any) => candidate.id === native?.runtime_capability_id)
        : undefined;
      const claims = new Map(Array.isArray(runtime?.claims) ? runtime.claims.map((claim: any) => [claim.namespace, claim.value]) : []);
      owners.push({
        artifact: definition.artifact,
        factory_id: native?.factory_id,
        descriptor_codec_id: codec.id,
        runtime_capability_id: native?.runtime_capability_id,
        artifact_kind: native?.artifact_kind,
        document_schema: native?.document_schema,
        extension: native?.extension,
        pack_schema_sha256: native?.pack_schema_hash,
        runtimeAuthorized: runtime?.category === "codec" && claims.size === 2 && claims.get("codec") === native?.document_schema && claims.get("codec-extension") === `${Buffer.byteLength(native?.document_schema ?? "", "utf8")}:${native?.document_schema}:${native?.extension}`,
      });
    }
  }
  const sha256 = async (bytes: Uint8Array): Promise<string> => Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex");
  const protocolDigests = new Map<string, string>();
  const stdioRoot = resolve(dirname(projectionPath), "../..");
  for (const receipt of projection.receipts) protocolDigests.set(receipt.protocol_path, await sha256(readFileSync(join(stdioRoot, receipt.protocol_path))));
  const componentDigest = await sha256(Buffer.from(fixture.attestation.componentHex, "hex"));
  const descriptorDigest = await sha256(Buffer.from(fixture.attestation.descriptorProjectionHex, "hex"));
  type Candidate = {
    owners: NativeOpenableOwnerReceipt[];
    projected: NativeOpenableProjectionReceipt[];
    targets: any[];
    componentSha256: string;
    descriptorSha256: string;
  };
  const valid = (candidate: Candidate): boolean => {
    if (candidate.owners.length !== fixture.receiptCount || candidate.projected.length !== fixture.receiptCount || candidate.targets.length !== 1) return false;
    const unique = (values: string[]): boolean => values.length === new Set(values).size;
    if (!unique(candidate.owners.map((row) => row.factory_id)) || !unique(candidate.owners.map((row) => row.descriptor_codec_id)) || !unique(candidate.projected.map((row) => row.factory_id))) return false;
    if (candidate.projected.some((row, index) => index > 0 && candidate.projected[index - 1]!.factory_id.localeCompare(row.factory_id) >= 0)) return false;
    if (candidate.componentSha256 !== componentDigest || candidate.descriptorSha256 !== descriptorDigest) return false;
    const ownerByFactory = new Map(candidate.owners.map((row) => [row.factory_id, row]));
    for (const row of candidate.projected) {
      const owner = ownerByFactory.get(row.factory_id);
      if (!owner?.runtimeAuthorized || protocolDigests.get(row.protocol_path) !== row.pack_schema_sha256) return false;
      for (const key of ["artifact", "factory_id", "descriptor_codec_id", "runtime_capability_id", "artifact_kind", "document_schema", "extension", "pack_schema_sha256"] as const) if (owner[key] !== row[key]) return false;
    }
    const target = candidate.targets[0];
    const json = candidate.projected.find((row) => row.factory_id === "stdio.native.json.v1");
    return Boolean(
      json &&
        target.artifactKind === json.artifact_kind &&
        target.artifactSchema === json.document_schema &&
        target.packSchemaHash === json.pack_schema_sha256 &&
        target.surfaceId === "s.stdio.json@rfc8259/*#viewer" &&
        target.appId === target.surfaceId &&
        target.windowKindId === "framework.window.tree" &&
        target.role === "viewer" &&
        target.rendererTarget === "wasm",
    );
  };
  const baseline = (): Candidate => ({
    owners: structuredClone(owners),
    projected: structuredClone(projection.receipts),
    targets: [structuredClone(fixture.openTarget)],
    componentSha256: fixture.attestation.componentSha256,
    descriptorSha256: fixture.attestation.descriptorProjectionSha256,
  });
  if (!valid(baseline())) throw new Error("native-openable positive owner/projection/target bijection was denied");
  for (const hostile of fixture.hostileCases as { name: string; mutation: string; outcome: "denied"; publishedTargets: 0 }[]) {
    const candidate = baseline();
    switch (hostile.mutation) {
      case "missing-owner": candidate.owners.pop(); break;
      case "extra-projection": candidate.projected.push({ ...candidate.projected[0]!, artifact: "foreign", factory_id: "stdio.native.foreign.v1" }); break;
      case "duplicate-factory": candidate.owners[1]!.factory_id = candidate.owners[0]!.factory_id; break;
      case "duplicate-descriptor-codec": candidate.owners[1]!.descriptor_codec_id = candidate.owners[0]!.descriptor_codec_id; break;
      case "missing-runtime-capability": candidate.owners[0]!.runtimeAuthorized = false; break;
      case "wrong-protocol-hash": candidate.projected[0]!.pack_schema_sha256 = "11".repeat(32); break;
      case "zero-protocol-hash": candidate.projected[0]!.pack_schema_sha256 = "00".repeat(32); break;
      case "wrong-component-hash": candidate.componentSha256 = "11".repeat(32); break;
      case "wrong-descriptor-hash": candidate.descriptorSha256 = "11".repeat(32); break;
      case "wrong-surface": candidate.targets[0]!.surfaceId = "s.stdio.json@rfc8259/*#foreign"; break;
      case "wrong-role": candidate.targets[0]!.role = "editor"; break;
      case "wrong-renderer": candidate.targets[0]!.rendererTarget = "wgpu"; break;
      case "duplicate-target": candidate.targets.push(structuredClone(candidate.targets[0])); break;
      default: throw new Error(`native-openable unknown hostile mutation ${hostile.mutation}`);
    }
    if (valid(candidate) || hostile.outcome !== "denied" || hostile.publishedTargets !== 0) throw new Error(`native-openable hostile case admitted or partially published: ${hostile.name}`);
  }
  console.log(`native-openable-neutral-oracle: AJV=2 owner-receipts=${owners.length} protocol-webcrypto=${protocolDigests.size} targets=1 hostile-denied=${fixture.hostileCases.length} no-partial=${fixture.hostileCases.length}`);
}

class NativeOpenableCatalogProviderCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveNativeOpenableCatalogProviderFixture(this.repoRoot);
    if (process.argv.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "268435456" },
      groups: [
        { package: "semio-s-plugin-stdio", target: { kind: "test", name: "native_openable_provider" }, laws: [
          "native_composition_and_validation_claims_are_disjoint_but_each_exclusive",
          "artifact_owned_native_codec_receipts_form_one_complete_static_bijection",
        ] },
        { package: "semio-hub", target: { kind: "lib", name: "semio_hub" }, cargoArgs: ["--features", "native-artifact-execution"], laws: [
          "native_openable_provider_consumes_exact_complete_stdio_factory_closure",
          "native_openable_provider_rejects_missing_extra_and_duplicate_receipts_without_publication",
          "native_openable_provider_rejects_identity_hash_schema_and_factory_substitution",
          "descriptor_owned_surface_is_required_before_any_catalog_or_codec_publication",
        ] },
        { package: "semio-hub", target: { kind: "bin", name: "os-hub" }, cargoArgs: ["--features", "native-artifact-execution"], laws: ["native_openable_stdio_provider_is_the_only_atomic_readiness_transition"] },
      ],
      progress(event) { console.log(`native-openable-provider ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`native-openable-provider-receipt: ${JSON.stringify(receipt)}`);
    console.log(`native-openable-catalog-provider-laws: passed=${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)}`);
    console.log("native-openable-catalog-provider-check: complete stdio closure, descriptor-owned JSON viewer and isolated readiness journey; no all-plugin or client-mount claim");
  }
}

class NativeCatalogSelectionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("unsupported native catalog selection argument");
    runCmd("bun", [join(this.repoRoot, "📜️script.ts"), "nx", "run", "@semio-tech/plugin-registry:native-catalog-selection-check", "--skip-nx-cache"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-hub", target: { kind: "lib", name: "semio_hub" }, laws: [
        "selected_native_providers_are_descriptor_verified_dependency_first_and_only_selected",
        "selected_native_provider_failure_substitution_and_conflict_publish_no_partial_closure",
        "selected_native_provider_descriptor_and_cancellation_fences_precede_publication",
      ] }],
      progress(event) { console.log(`native-catalog-selection ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    console.log(`native-catalog-selection-laws: ${JSON.stringify(receipts)}`);
    console.log("native-catalog-selection-check: selected-only loader admission; no VCS provider, immutable bundle or client activation claim");
  }
}

class OpenPlanCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveDocumentOpenPlanFixture(this.repoRoot);
    const env = { ...process.env, RUST_MIN_STACK: "268435456" };
    const exactLaw = (target: string[], suffix: string): string => {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--all-features", ...target, suffix, "--", "--list"], { cwd: this.root, env, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) {
        const related = listed.stdout
          .split("\n")
          .filter((line) => line.includes("document_open"))
          .join(",");
        const diagnostic = listed.stderr.trim().slice(-4_000);
        throw new Error(`open-plan-check expected exactly one ${suffix} law, selected ${matches.length}; status=${listed.status}; related=${related || "<none>"}; diagnostic=${diagnostic || "<none>"}`);
      }
      return matches[0]!;
    };
    const schemaTarget = ["-p", "semio-framework-os-kernel", "--lib"];
    const schemaLaw = exactLaw(schemaTarget, "document_open_plan_v1_matches_language_neutral_fixture");
    const catalogTarget = ["-p", "semio-hub", "--lib"];
    const catalogLaw = exactLaw(catalogTarget, "verified_trusted_catalog_document_open_generation_and_resolution_are_exact");
    const ledgerLaw = exactLaw(["--bin", "os-hub"], "document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped");
    const revocationLaw = exactLaw(["--bin", "os-hub"], "document_open_plan_admin_revocation_invalidates_session_and_share_bindings");
    const exchangeLaw = exactLaw(["--bin", "os-hub"], "document_open_plan_receipt_exchange_mints_one_exact_bounded_socket_grant");
    const routeLaw = exactLaw(["--bin", "os-hub"], "document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use");
    const wipeLaw = exactLaw(["--bin", "os-hub"], "document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes");
    const issueLaw = exactLaw(["--bin", "os-hub"], "document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable");
    const consumeLaw = exactLaw(["--bin", "os-hub"], "document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint");
    console.log(`document-open-plan-laws: schema=${schemaLaw} catalog=${catalogLaw} ledger=${ledgerLaw} revocation=${revocationLaw} exchange=${exchangeLaw} route=${routeLaw} wipe=${wipeLaw} issue=${issueLaw} consume=${consumeLaw}`);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", ...schemaTarget, schemaLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", ...catalogTarget, catalogLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", ledgerLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", revocationLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", exchangeLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", routeLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", wipeLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", issueLaw, "--", "--exact", "--test-threads=1"], this.root, env);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", consumeLaw, "--", "--exact", "--test-threads=1"], this.root, env);
  }
}

class OpenPlanServerCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("unsupported open-plan server argument");
    await proveDocumentOpenPlanFixture(this.repoRoot);
    if (segments.includes("--oracle-only")) return;
    const env = { ...process.env, RUST_MIN_STACK: "268435456" };
    const exactLaw = (target: string[], suffix: string): string => {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", ...target, suffix, "--", "--list"], { cwd: this.root, env, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) {
        const diagnostic = listed.stderr.trim().slice(-4_000);
        throw new Error(`open-plan-server-check expected exactly one ${suffix} law, selected ${matches.length}; status=${listed.status}; diagnostic=${diagnostic || "<none>"}`);
      }
      return matches[0]!;
    };
    const laws = [
      { target: ["--lib"], suffix: "verified_trusted_catalog_document_open_generation_and_resolution_are_exact" },
      { target: ["--bin", "os-hub"], suffix: "document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped" },
      { target: ["--bin", "os-hub"], suffix: "document_open_plan_admin_revocation_invalidates_session_and_share_bindings" },
      { target: ["--bin", "os-hub"], suffix: "document_open_plan_receipt_exchange_mints_one_exact_bounded_socket_grant" },
      { target: ["--bin", "os-hub"], suffix: "document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use" },
      { target: ["--bin", "os-hub"], suffix: "document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes" },
      { target: ["--bin", "os-hub"], suffix: "document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable" },
      { target: ["--bin", "os-hub"], suffix: "document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint" },
    ].map((law) => ({ ...law, name: exactLaw(law.target, law.suffix) }));
    console.log(`document-open-plan-server-laws: qualification=default-feature-subset exact=${laws.length} laws=${laws.map((law) => law.name).join(",")}`);
    for (const law of laws) runCargo(["test", "--manifest-path", "Cargo.toml", ...law.target, law.name, "--", "--exact", "--test-threads=1"], this.root, env);
    console.log("document-open-plan-server-check: default-feature subset passed; kernel all-features schema qualification remains separate");
  }
}

type ExecutionTargetLeaseFixture = {
  readonly schema: string;
  readonly version: 1;
  readonly nowMs: number;
  readonly hubOrigin: string;
  readonly intent: Record<string, any>;
  readonly plan: Record<string, any>;
  readonly manifest: Record<string, any>;
  readonly socketGrant: Record<string, any>;
  readonly componentHex: string;
  readonly descriptorHex: string;
  readonly expected: Record<string, any>;
  readonly hostile: readonly { readonly name: string; readonly stage: string; readonly kind: string; readonly path?: string; readonly value?: unknown; readonly expected: "unpublished" }[];
};

/** 🪪️ Independent hand-written full-field lease relation. It imports no production comparison and is
 * the corpus oracle's only admission decision. */
function executionTargetLeaseFieldsEqual(left: Record<string, any>, right: Record<string, any>): boolean {
  const scalarPaths = [
    "schema", "version", "scope.spaceId", "scope.documentId", "descriptorDigestV1", "catalog.generationId",
    "package.pluginId", "package.packageId", "package.version", "package.componentSha256", "package.componentBlake3", "package.descriptorByteSha256",
    "component.sha256", "component.blake3", "component.byteLength", "descriptor.sha256", "descriptor.byteLength",
    "artifact.kind", "artifact.schema", "artifact.packSchemaHash",
    "parentDialect.artifactKind", "parentDialect.standard", "parentDialect.subset",
    "surface.surfaceId", "surface.appId", "surface.windowKindId", "surface.role", "surface.rendererTarget",
    "grant.read", "grant.write", "grant.observe",
    "checkpoint.checkpointId", "checkpoint.descriptorDigestV1", "checkpoint.aggregateSha256",
    "checkpoint.baselineFrontier.documentId", "checkpoint.baselineFrontier.headEditOrdinal", "checkpoint.baselineFrontier.headEditId", "checkpoint.baselineFrontier.lastCommitSeq",
    "revalidation.directoryRevision", "revalidation.membershipGeneration", "revalidation.sessionGeneration", "revalidation.shareGeneration",
  ];
  const read = (source: Record<string, any>, path: string): unknown => path.split(".").reduce<any>((cursor, segment) => (cursor === undefined || cursor === null ? undefined : cursor[segment]), source);
  return scalarPaths.every((path) => read(left, path) === read(right, path))
    && JSON.stringify(read(left, "checkpoint.baselineFrontier.chainHash")) === JSON.stringify(read(right, "checkpoint.baselineFrontier.chainHash"));
}

/** 🪪️ Independent structural admission of one lease-fields value: every identity, byte bound and
 * grant/role invariant, decided without importing the production parser. */
function executionTargetLeaseFieldsAdmissible(candidate: Record<string, any>, expected: Record<string, any>): boolean {
  const hash = (value: unknown): boolean => typeof value === "string" && /^[0-9a-f]{64}$/u.test(value) && !/^0{64}$/u.test(value);
  const text = (value: unknown): boolean => typeof value === "string" && value.length > 0 && Buffer.byteLength(value, "utf8") <= 256 && ![...value].some((character) => character.codePointAt(0)! < 0x20 || character.codePointAt(0)! === 0x7f);
  const length = (value: unknown, maximum: number): boolean => typeof value === "number" && Number.isSafeInteger(value) && value >= 1 && value <= maximum;
  const generation = (value: unknown): boolean => typeof value === "number" && Number.isSafeInteger(value) && value >= 1 && value <= 9_007_199_254_740_991;
  return candidate.schema === "semio.os.document-execution-target-lease/v1"
    && candidate.version === 1
    && [candidate.scope?.spaceId, candidate.scope?.documentId, candidate.package?.pluginId, candidate.package?.packageId, candidate.package?.version, candidate.artifact?.kind, candidate.artifact?.schema, candidate.surface?.surfaceId, candidate.surface?.appId, candidate.surface?.windowKindId].every(text)
    && [candidate.descriptorDigestV1, candidate.catalog?.generationId, candidate.package?.componentSha256, candidate.package?.componentBlake3, candidate.package?.descriptorByteSha256, candidate.artifact?.packSchemaHash, candidate.component?.sha256, candidate.component?.blake3, candidate.descriptor?.sha256].every(hash)
    && candidate.component.sha256 === candidate.package.componentSha256
    && candidate.component.blake3 === candidate.package.componentBlake3
    && candidate.descriptor.sha256 === candidate.package.descriptorByteSha256
    && length(candidate.component?.byteLength, expected.componentMaxBytes)
    && length(candidate.descriptor?.byteLength, expected.descriptorMaxBytes)
    && candidate.parentDialect?.artifactKind === candidate.artifact?.kind
    && [candidate.parentDialect?.artifactKind, candidate.parentDialect?.standard, candidate.parentDialect?.subset].every((value) => text(value) && String(value).trim() === value)
    && candidate.grant?.read === true
    && candidate.grant?.observe === true
    && typeof candidate.grant?.write === "boolean"
    && ["viewer", "editor"].includes(candidate.surface?.role)
    && ["react", "wgpu", "wasm"].includes(candidate.surface?.rendererTarget)
    && candidate.grant.write === (candidate.surface.role === "editor")
    && (candidate.checkpoint === undefined || (hash(candidate.checkpoint.checkpointId) && candidate.checkpoint.descriptorDigestV1 === candidate.descriptorDigestV1 && hash(candidate.checkpoint.aggregateSha256)))
    && generation(candidate.revalidation?.directoryRevision)
    && generation(candidate.revalidation?.membershipGeneration)
    && (candidate.revalidation?.sessionGeneration === undefined) !== (candidate.revalidation?.shareGeneration === undefined);
}

function executionTargetLeaseMutate(source: Record<string, any>, path: string, value: unknown): Record<string, any> {
  const candidate = structuredClone(source);
  const segments = path.split(".");
  let cursor: any = candidate;
  for (const segment of segments.slice(0, -1)) cursor = cursor[segment];
  cursor[segments.at(-1)!] = value;
  return candidate;
}

/** 🤖️ Independent Node state machine for one browser install: it walks manifest → component →
 * descriptor → verify → exchange with its own byte reader and hashers, and answers `published` only
 * when every field and byte agrees. It never imports the browser worker. */
function executionTargetLeaseInstall(
  fixture: ExecutionTargetLeaseFixture,
  input: { manifest: Record<string, any>; component: Buffer; declaredComponentLength: number; descriptor: Buffer; declaredDescriptorLength: number; planGeneration: string; cancelAt?: string; missing?: string },
  blake3Hex: (bytes: Uint8Array) => string,
): { readonly outcome: "published" | "unpublished"; readonly stage: string } {
  const stages = ["manifest", "component", "descriptor", "verify", "exchange"] as const;
  const cancel = input.cancelAt;
  for (const stage of stages) {
    if (cancel === stage) return { outcome: "unpublished", stage };
    if (input.missing === stage) return { outcome: "unpublished", stage };
    if (stage === "manifest") {
      if (!executionTargetLeaseFieldsAdmissible(input.manifest, fixture.expected)) return { outcome: "unpublished", stage };
      const projection = { ...structuredClone(fixture.plan), catalog: { generationId: input.planGeneration } } as Record<string, any>;
      const planFields = {
        schema: "semio.os.document-execution-target-lease/v1",
        version: 1,
        scope: projection.scope,
        descriptorDigestV1: projection.descriptorDigestV1,
        catalog: projection.catalog,
        package: projection.package,
        component: { sha256: projection.package.componentSha256, blake3: projection.package.componentBlake3, byteLength: input.manifest.component?.byteLength },
        descriptor: { sha256: projection.package.descriptorByteSha256, byteLength: input.manifest.descriptor?.byteLength },
        artifact: projection.artifact,
        parentDialect: projection.parentDialect,
        surface: projection.surface,
        grant: projection.grant,
        checkpoint: projection.checkpoint,
        revalidation: projection.revalidation,
      };
      if (!executionTargetLeaseFieldsEqual(planFields, input.manifest)) return { outcome: "unpublished", stage };
      continue;
    }
    if (stage === "component") {
      if (input.declaredComponentLength !== input.manifest.component.byteLength || input.declaredComponentLength > fixture.expected.componentMaxBytes || input.component.length !== input.declaredComponentLength) return { outcome: "unpublished", stage };
      continue;
    }
    if (stage === "descriptor") {
      if (input.declaredDescriptorLength !== input.manifest.descriptor.byteLength || input.declaredDescriptorLength > fixture.expected.descriptorMaxBytes || input.descriptor.length !== input.declaredDescriptorLength) return { outcome: "unpublished", stage };
      continue;
    }
    if (stage === "verify") {
      const componentSha256 = createHash("sha256").update(input.component).digest("hex");
      const descriptorSha256 = createHash("sha256").update(input.descriptor).digest("hex");
      if (componentSha256 !== input.manifest.component.sha256 || blake3Hex(input.component) !== input.manifest.component.blake3 || descriptorSha256 !== input.manifest.descriptor.sha256) return { outcome: "unpublished", stage };
      continue;
    }
    if (input.planGeneration !== input.manifest.catalog.generationId) return { outcome: "unpublished", stage };
  }
  return { outcome: "published", stage: "exchange" };
}

async function proveExecutionTargetLeaseCorpus(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1");
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as ExecutionTargetLeaseFixture;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ allErrors: true, strict: true });
  ajv.addSchema(schema);
  const validate = ajv.getSchema(schema.$id)!;
  if (!validate(fixture)) throw new Error(`execution target lease corpus invalid: ${JSON.stringify(validate.errors)}`);
  const validateFields = ajv.getSchema(`${schema.$id}#/$defs/leaseFields`)!;
  const { blake3Hex } = await import(join(repoRoot, "🧰️framework/🔨️modules/🔏️hash/🟦️.ts"));
  const component = Buffer.from(fixture.componentHex, "hex");
  const descriptor = Buffer.from(fixture.descriptorHex, "hex");
  if (component.length === 0 || descriptor.length === 0) throw new Error("execution target lease corpus must pin non-empty component and descriptor bytes");
  const componentSha256 = createHash("sha256").update(component).digest("hex");
  const descriptorSha256 = createHash("sha256").update(descriptor).digest("hex");
  const componentBlake3 = blake3Hex(component);
  const webComponentSha256 = Buffer.from(await webcrypto.subtle.digest("SHA-256", component)).toString("hex");
  if (componentSha256 !== webComponentSha256) throw new Error("execution target lease corpus Node and WebCrypto SHA-256 disagree");
  if (blake3Hex(Buffer.from("abc", "utf8")) !== "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85") throw new Error("first-party BLAKE3 known-answer vector regressed");
  if (componentSha256 !== fixture.manifest.component.sha256 || componentBlake3 !== fixture.manifest.component.blake3 || descriptorSha256 !== fixture.manifest.descriptor.sha256) throw new Error("execution target lease corpus digests do not match its exact bytes");
  if (component.length !== fixture.manifest.component.byteLength || descriptor.length !== fixture.manifest.descriptor.byteLength) throw new Error("execution target lease corpus byte lengths drifted");
  if (fixture.plan.surface.rendererTarget !== "wasm" || fixture.manifest.surface.role !== "viewer" || fixture.manifest.grant.write !== false) throw new Error("execution target lease corpus lost its read-only GIS Map wasm viewer positive");

  const generationA = fixture.expected.rotation.generationA;
  const baseline = { manifest: fixture.manifest as Record<string, any>, component, declaredComponentLength: component.length, descriptor, declaredDescriptorLength: descriptor.length, planGeneration: generationA };
  const positive = executionTargetLeaseInstall(fixture, baseline, blake3Hex);
  if (positive.outcome !== "published") throw new Error(`execution target lease corpus positive vector was denied at ${positive.stage}`);

  let manifestFields = 0;
  let byteVectors = 0;
  let lifecycleVectors = 0;
  for (const vector of fixture.hostile) {
    if (vector.expected !== "unpublished") throw new Error(`execution target lease corpus hostile ${vector.name} is not an unpublished expectation`);
    if (vector.kind === "manifest-field") {
      const mutated = executionTargetLeaseMutate(fixture.manifest, vector.path!, vector.value);
      const admitted = validateFields(mutated) && executionTargetLeaseInstall(fixture, { ...baseline, manifest: mutated }, blake3Hex).outcome === "published";
      if (admitted) throw new Error(`execution target lease corpus admitted single-field substitution ${vector.name}`);
      manifestFields += 1;
      continue;
    }
    if (vector.kind === "cancel" || vector.kind === "deadline" || vector.kind === "reconnect-after-invalidation" || vector.kind === "viewer-write" || vector.kind === "caller-url" || vector.kind === "caller-path" || vector.kind === "caller-module" || vector.kind === "stale-plan") {
      if (vector.kind === "cancel" || vector.kind === "deadline") {
        const stage = vector.kind === "deadline" ? "component" : vector.stage;
        if (executionTargetLeaseInstall(fixture, { ...baseline, cancelAt: stage }, blake3Hex).outcome === "published") throw new Error(`execution target lease corpus published through ${vector.name}`);
      }
      if (vector.kind === "stale-plan" && executionTargetLeaseInstall(fixture, { ...baseline, planGeneration: fixture.expected.rotation.generationB }, blake3Hex).outcome === "published") throw new Error("execution target lease corpus exchanged a stale rotated plan");
      if (vector.kind === "viewer-write" && fixture.expected.viewerWriteRejectedLocally !== true) throw new Error("execution target lease corpus lost its local viewer write rejection");
      if ((vector.kind === "caller-url" || vector.kind === "caller-path" || vector.kind === "caller-module") && fixture.expected.assetPaths.includes(String(vector.value))) throw new Error(`execution target lease corpus accepted caller substitution ${vector.name}`);
      lifecycleVectors += 1;
      continue;
    }
    const mutated = { ...baseline };
    if (vector.kind === "component-bytes" || vector.kind === "mixed-generation") mutated.component = Buffer.concat([Buffer.from([component[0]! ^ 0xff]), component.subarray(1)]);
    else if (vector.kind === "component-truncated") mutated.component = component.subarray(0, component.length - 1);
    else if (vector.kind === "component-extra-byte") mutated.component = Buffer.concat([component, Buffer.from([7])]);
    else if (vector.kind === "component-max-plus-one") mutated.declaredComponentLength = Number(vector.value);
    else if (vector.kind === "descriptor-bytes" || vector.kind === "descriptor-self-hash" || vector.kind === "descriptor-noncanonical") mutated.descriptor = Buffer.concat([descriptor.subarray(0, descriptor.length - 1), Buffer.from([descriptor.at(-1)! ^ 0xff])]);
    else if (vector.kind === "descriptor-trailing-byte") mutated.descriptor = Buffer.concat([descriptor, Buffer.from([0])]);
    else if (vector.kind === "descriptor-max-plus-one") mutated.declaredDescriptorLength = Number(vector.value);
    else if (vector.kind === "missing-body") mutated.missing = vector.stage;
    else throw new Error(`execution target lease corpus unknown hostile kind ${vector.kind}`);
    if (executionTargetLeaseInstall(fixture, mutated, blake3Hex).outcome === "published") throw new Error(`execution target lease corpus published through ${vector.name}`);
    byteVectors += 1;
  }
  if (manifestFields < 30) throw new Error(`execution target lease corpus lost single-field substitutions: ${manifestFields}`);
  const statusCodes = Object.keys(fixture.expected.status).sort();
  if (JSON.stringify(statusCodes) !== JSON.stringify(["cancelled", "integrity-failed", "renderer-unavailable", "stale", "verifying"])) throw new Error("execution target lease corpus status vocabulary drifted");
  for (const [code, text] of Object.entries<{ en: string; de: string }>(fixture.expected.status)) {
    if (!text.en || !text.de || text.en === text.de) throw new Error(`execution target lease corpus status ${code} is not explicitly bilingual`);
    for (const fragment of fixture.expected.forbiddenStatusFragments) if (text.en.includes(fragment) || text.de.includes(fragment)) throw new Error(`execution target lease corpus status ${code} leaked ${fragment}`);
    const role = fixture.expected.statusRoles[code];
    if (role !== (code === "verifying" ? "status" : "alert")) throw new Error(`execution target lease corpus status ${code} has the wrong live-region role`);
  }
  if (Object.values(fixture.expected.rendererClaims).some((claim) => claim !== false)) throw new Error("execution target lease corpus claims a renderer it does not have");
  console.log(`execution-target-lease-oracle: ajv=1 positive=1 manifest-fields=${manifestFields} byte-vectors=${byteVectors} lifecycle=${lifecycleVectors} hostile=${fixture.hostile.length} component-bytes=${component.length} descriptor-bytes=${descriptor.length} node+webcrypto-sha256=agree first-party-blake3=known-answer status=${statusCodes.length} passed`);
}

function proveExecutionTargetLeaseSource(repoRoot: string): void {
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  const region = worker.slice(worker.indexOf("//#region 🪪️ExecutionTargetLease"), worker.indexOf("//#endregion 🪪️ExecutionTargetLease"));
  if (region.length === 0) throw new Error("browser execution-target lease region is absent");
  for (const forbidden of ["loadPluginModule", "ActivationRegistry", "load_wasm_plugins", "attach_backbone"]) {
    if (region.includes(forbidden)) throw new Error(`browser execution-target lease region reached ${forbidden}`);
  }
  if (!region.includes("crypto.subtle.digest(\"SHA-256\"") || !region.includes("blake3Hex(")) throw new Error("browser execution-target lease lost its Web Crypto SHA-256 or first-party BLAKE3 verification");
  if (!region.includes("state.docAbort.signal")) throw new Error("browser execution-target lease no longer shares the document cancellation scope");
  if (!worker.includes("plan.surface.rendererTarget !== \"react\" && (leaseFields === undefined")) throw new Error("browser plan authority no longer gates a non-react renderer on a live lease");
  const bin = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");
  for (const asset of ["manifest", "component", "descriptor"]) {
    if (!bin.includes(`/spaces/{space_id}/documents/{id}/execution-target/${asset}`)) throw new Error(`hub execution-target ${asset} route is not mounted`);
  }
  if (!bin.includes("assets_for_current_selection(&descriptor, intent.requested_surface_id.as_deref(), writable, &generation_id)")) throw new Error("hub execution-target routes no longer resolve through the exact-selection accessor");
  const catalog = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"), "utf8");
  if (!catalog.includes("pub fn assets_for_current_selection(") || !catalog.includes("if current_generation != self.generation_id")) throw new Error("trusted catalog exact-selection accessor is absent or not generation-bound");
  const client = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  if (client.includes("matches_surface") || client.includes("DocumentSocketSurfaceExpectationV1")) throw new Error("native directory client kept a partial surface predicate");
  const sync = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"), "utf8");
  if (!sync.includes("authority.matches_lease_fields(lease)")) throw new Error("native reconnect no longer compares the complete lease fields");
  console.log("execution-target-lease-source: browser lease region=renderer-free hub routes=3 accessor=generation-bound native=full-field passed");
}

class ExecutionTargetLeaseCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some((segment) => segment !== "--native")) throw new Error("unsupported execution-target-lease argument");
    await proveExecutionTargetLeaseCorpus(this.repoRoot);
    proveExecutionTargetLeaseSource(this.repoRoot);
    if (!segments.includes("--native")) return;
    const env = { ...process.env, RUST_MIN_STACK: "268435456" };
    const exactLaw = (target: string[], suffix: string): string => {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", ...target, suffix, "--", "--list"], { cwd: this.root, env, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) {
        const diagnostic = listed.stderr.trim().slice(-4_000);
        throw new Error(`execution-target-lease-check expected exactly one ${suffix} law, selected ${matches.length}; status=${listed.status}; diagnostic=${diagnostic || "<none>"}`);
      }
      return matches[0]!;
    };
    const laws = [
      { target: ["--lib"], suffix: "selected_execution_target_assets_are_generation_and_digest_bound" },
      { target: ["--bin", "os-hub"], suffix: "execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body" },
      { target: ["-p", "semio-framework-os-kernel", "--lib"], suffix: "execution_target_lease_compares_every_plan_and_verified_byte_field" },
    ].map((law) => ({ ...law, name: exactLaw(law.target, law.suffix) }));
    console.log(`execution-target-lease-laws: qualification=default-feature-subset exact=${laws.length} laws=${laws.map((law) => law.name).join(",")}`);
    for (const law of laws) runCargo(["test", "--manifest-path", "Cargo.toml", ...law.target, law.name, "--", "--exact", "--test-threads=1"], this.root, env);
    console.log("execution-target-lease-check: neutral corpus, source boundary and current native laws passed");
  }
}

class ExecutionTargetLeaseBrowserCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveExecutionTargetLeaseCorpus(this.repoRoot);
    proveExecutionTargetLeaseSource(this.repoRoot);
    runCmd("bun", ["nx", "run", "@semio-tech/framework-os:test-long", "--skip-nx-cache", "--", "--run", "-t", "browser execution target lease|browser GIS viewer exposes localized renderer-unavailable"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    console.log("execution-target-lease-browser-check: neutral corpus, source boundary and browser Worker verify/reject/renderer-unavailable runtime passed");
  }
}

class BrowserDocumentOpenCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = await proveBrowserDocumentOpenFixture(this.repoRoot);
    await proveBrowserDocumentOpenRuntime(this.repoRoot, fixture);
    runCmd("bun", ["nx", "run", "@semio-tech/framework-os:test-quick", "--skip-nx-cache", "--", "--run", "-t", "browser document open"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCmd("bun", ["./📜️script.ts", "open-plan-server-check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("browser-document-open-check: neutral oracle, browser Worker D1 issue/exchange/WebSocket runtime, Session-gated activation, hostile bounds/cancellation/redaction, and current server laws passed");
  }
}

type SpacePublicBoundaryFixture = {
  readonly positives: Readonly<Record<"anonymous" | "publicNonmember" | "member" | "author", Record<string, unknown>>>;
  readonly forbiddenPublicKeys: readonly string[];
  readonly hostileMutations: readonly { readonly name: string; readonly path: readonly (string | number)[]; readonly value: unknown }[];
  readonly rawPublicEvent: Record<string, unknown>;
};

function mutateSpacePublicProjection(source: Record<string, unknown>, path: readonly (string | number)[], value: unknown): Record<string, unknown> {
  const candidate = structuredClone(source) as Record<string, unknown>;
  let cursor: any = candidate;
  for (const segment of path.slice(0, -1)) cursor = cursor[segment];
  cursor[path[path.length - 1]!] = value;
  return candidate;
}

async function proveSpacePublicBoundaryFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧫️fixtures/🏛️public-space-detail-v1");
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as SpacePublicBoundaryFixture;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ allErrors: true, strict: true });
  ajv.addSchema(schema);
  const validateFixture = ajv.getSchema(schema.$id);
  const validatePublic = ajv.getSchema(`${schema.$id}#/$defs/publicDetail`);
  const validateMember = ajv.getSchema(`${schema.$id}#/$defs/memberDetail`);
  const validateAuthor = ajv.getSchema(`${schema.$id}#/$defs/authorDetail`);
  if (!validateFixture?.(fixture)) throw new Error(`space public boundary fixture invalid: ${JSON.stringify(validateFixture?.errors)}`);
  if (!validatePublic?.(fixture.positives.anonymous) || !validatePublic(fixture.positives.publicNonmember) || !validateMember?.(fixture.positives.member) || !validateAuthor?.(fixture.positives.author)) throw new Error("space public boundary positive projection drift");
  const publicBytes = JSON.stringify([fixture.positives.anonymous, fixture.positives.publicNonmember]);
  for (const key of fixture.forbiddenPublicKeys) if (publicBytes.includes(`\"${key}\"`)) throw new Error(`space public boundary positive leaked ${key}`);
  for (const vector of fixture.hostileMutations) {
    const candidate = mutateSpacePublicProjection(fixture.positives.anonymous, vector.path, vector.value);
    if (validatePublic?.(candidate)) throw new Error(`space public boundary admitted hostile ${vector.name}`);
  }
  if (validatePublic?.(fixture.rawPublicEvent)) throw new Error("space public boundary admitted a raw DirectoryEvent as public projection");
  const raw = JSON.stringify(fixture.rawPublicEvent);
  if (!raw.includes('"actor"') || !raw.includes('"hlc"') || !raw.includes('"userId"')) throw new Error("space public boundary raw event hostile vector lost identity fields");
  console.log(`space-public-boundary-oracle: ajv=1 positives=4 hostile=${fixture.hostileMutations.length} forbidden=${fixture.forbiddenPublicKeys.length} raw-event=denied passed`);
}

class SpacePublicBoundaryCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveSpacePublicBoundaryFixture(this.repoRoot);
    const target = ["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"];
    const suffixes = [
      "space_public_boundary_real_routes_emit_discriminated_public_member_author_and_private_404",
      "space_public_boundary_public_event_route_denies_raw_directory_events",
      "space_public_boundary_real_socket_denies_public_raw_events_and_member_telemetry",
    ];
    const listed = runProbe("cargo", [...target, "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
    const discovered = listed.stdout
      .split("\n")
      .filter((line) => line.endsWith(": test"))
      .map((line) => line.slice(0, -": test".length));
    const laws = suffixes.map((suffix) => {
      const matches = discovered.filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`space-public-boundary-check expected exactly one ${suffix}, selected ${matches.length}; status=${listed.status}; diagnostic=${listed.stderr.trim().slice(-4_000) || "<none>"}`);
      return matches[0]!;
    });
    console.log(`space-public-boundary-laws: exact=${laws.length} laws=${laws.join(",")}`);
    for (const law of laws) runCargo([...target, law, "--", "--exact", "--test-threads=1"], this.root);
    runCmd("bun", ["nx", "run", "@semio-tech/framework-os:test-quick", "--skip-nx-cache", "--", "--run", "-t", "DirectoryClient space public boundary"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root);
    console.log("space-public-boundary-check: hostile neutral projection oracle, exact real REST/socket laws, TypeScript discriminator/unknown-field law, and hub all-feature check passed");
  }
}

/** 💡️ Independent schema, SHA-256, bounds and lifecycle reference over the neutral ledger corpus. */
async function proveInferenceWalProofFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🧪️fixtures", "🧾️inference-wal-proof-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid WAL proof fixture: ${JSON.stringify(validate.errors)}`);
  const key = `v1:${Buffer.byteLength(fixture.scope.spaceId)}:${Buffer.byteLength(fixture.scope.documentId)}:${fixture.scope.spaceId}${fixture.scope.documentId}`;
  if (key !== fixture.documentKey || key !== fixture.command.documentId) throw new Error("WAL fixture full document key mismatch");
  const encoded: number[] = [];
  const integer = (value: number): void => {
    let remaining = BigInt(value);
    do { const byte = Number(remaining & 127n); remaining >>= 7n; encoded.push(byte | (remaining ? 128 : 0)); } while (remaining);
  };
  const bytes = (value: Buffer): void => { integer(value.byteLength); encoded.push(...value); };
  const text = (value: string): void => bytes(Buffer.from(value, "utf8"));
  const command = fixture.command;
  text(command.mutationId); text(command.documentId); text(command.actor);
  integer(command.dependencies.length); command.dependencies.forEach(text);
  text(command.diff.schema); bytes(Buffer.from(command.diff.payloadHex, "hex"));
  text(command.inverse.schema); bytes(Buffer.from(command.inverse.payloadHex, "hex"));
  integer(command.timestamp.actor); integer(command.timestamp.physicalMs); integer(command.timestamp.logical);
  const canonical = Buffer.from(encoded);
  if (canonical.toString("hex") !== fixture.encodedHex || createHash("sha256").update(canonical).digest("hex") !== fixture.commandHash) throw new Error("independent protocol envelope/hash mismatch");
  const ledger = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️fixtures", "🗺️gis-inference-job-v1", "🔣️.json"), "utf8"));
  if (ledger.outbox.commandHex !== fixture.encodedHex || ledger.outbox.commandHash !== fixture.commandHash || ledger.outbox.mutationId !== command.mutationId || ledger.outbox.jobId !== fixture.jobId || ledger.outbox.proposalHash !== fixture.proposalHash || ledger.identity.spaceId !== fixture.scope.spaceId || ledger.identity.documentId !== fixture.scope.documentId) throw new Error("ledger and committed-WAL fixtures disagree on exact command authority");
  if (createHash("sha256").update(`semio.hub.inference-approval-mutation/v1\0${fixture.jobId}\0${fixture.proposalHash}`).digest("hex").slice(0, 32) !== command.mutationId) throw new Error("committed-WAL mutation does not bind the exact job and proposal");
  for (const mismatch of fixture.bindingMismatches) if (mismatch.jobId === fixture.jobId && mismatch.proposalHash === fixture.proposalHash) throw new Error("hostile witness binding was not distinct");
  const outcome = (trace: any): string => {
    if (trace.cancelAfterRecords === 0) return "cancelled";
    if (trace.tornTail) return "invalid";
    const records = trace.flushed === false ? [] : trace.records;
    let active: { id: number; count: number; matches: number } | undefined;
    let matches = 0;
    for (let index = 0; index < records.length; index++) {
      if (trace.cancelAfterRecords === index) return "cancelled";
      if (index >= (trace.maximumRecords ?? fixture.maximumRecords)) return "bounds";
      const record = records[index];
      if (record.kind === "begin") {
        if (active) return "invalid";
        active = { id: record.txId, count: 0, matches: 0 };
      } else if (record.kind === "command") {
        if (!active) return "invalid";
        active.count++;
        const bytes = Buffer.from(canonical);
        if (record.bytes === "different") bytes[1] ^= 1;
        if (record.bytes === "altered-target") bytes[bytes.length - 1] ^= 1;
        if (createHash("sha256").update(bytes).digest("hex") === fixture.commandHash) active.matches++;
      } else {
        if (!active || active.id !== record.txId) return "invalid";
        if (record.kind === "commit") {
          if (active.count !== record.recordCount) return "invalid";
          matches += active.matches;
          if (matches > 1) return "invalid";
        }
        active = undefined;
      }
    }
    if (trace.cancelAfterRecords === records.length) return "cancelled";
    if (active) return "invalid";
    if ((trace.observedGeneration ?? fixture.generation) !== fixture.generation || (trace.spaceId ?? fixture.scope.spaceId) !== fixture.scope.spaceId) return "stale";
    return matches === 1 ? "verified" : "absent";
  };
  for (const trace of fixture.traces) {
    const actual = outcome(trace);
    if (actual !== trace.expected) throw new Error(`WAL proof trace mismatch: ${trace.name}`);
    if (trace.reusableAfterInvalidation !== undefined && (actual !== "verified" || trace.reusableAfterInvalidation !== (fixture.generation === 0))) throw new Error("invalidated WAL witness remained reusable");
  }
  for (const owner of fixture.ownership) {
    const expected = owner.interrupt === "deadline" ? "expired" : "cancelled";
    if (owner.expected !== expected || owner.heldActive !== 1 || owner.releasedActive !== 0 || owner.stoppedProgress !== 1) throw new Error("WAL interruption must retain the owner until explicit close");
  }
  console.log(`inference-wal-proof-oracle: traces=${fixture.traces.length} ownership=${fixture.ownership.length} binding-hostile=${fixture.bindingMismatches.length} protocol-envelope=1 node-sha256=1; committed-WAL runtime still required`);
}

async function proveInferenceWalChainFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🧪️fixtures", "⛓️inference-wal-chain-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const proof = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️fixtures", "🧾️inference-wal-proof-v1", "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid WAL chain fixture: ${JSON.stringify(validate.errors)}`);
  const { blake3Hex } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"));
  const digest = (bytes: Buffer): Buffer => Buffer.from(blake3Hex(bytes), "hex");
  const crc = (bytes: Buffer): number => {
    let value = 0xffffffff;
    for (const byte of bytes) { value ^= byte; for (let bit = 0; bit < 8; bit++) value = (value >>> 1) ^ ((value & 1) ? 0x82f63b78 : 0); }
    return (value ^ 0xffffffff) >>> 0;
  };
  const integer = (value: number, width: 4 | 8): Buffer => {
    const bytes = Buffer.alloc(width);
    if (width === 4) bytes.writeUInt32LE(value); else bytes.writeBigUInt64LE(BigInt(value));
    return bytes;
  };
  const varint = (value: number): Buffer => {
    const bytes: number[] = [];
    do { const byte = value % 128; value = Math.floor(value / 128); bytes.push(byte | (value ? 128 : 0)); } while (value);
    return Buffer.from(bytes);
  };
  type Frame = { start: number; body: number; end: number; next: number; kind: number };
  const frames = (bytes: Buffer): Frame[] => {
    const output: Frame[] = [];
    let at = 32;
    while (at < bytes.length) {
      const start = at; let length = 0, scale = 1, byte = 128;
      for (let count = 0; byte & 128; count++) {
        if (count === 10 || at >= bytes.length) throw new Error("invalid oracle frame length");
        byte = bytes[at++]; length += (byte & 127) * scale; scale *= 128;
      }
      const end = at + length, next = end + 8;
      if (!Number.isSafeInteger(length) || length < 2 || next > bytes.length || bytes.readUInt32LE(end + 4) !== next - start) throw new Error("invalid oracle frame bounds");
      output.push({ start, body: at, end, next, kind: bytes[at] }); at = next;
    }
    return output;
  };
  const frame = (kind: number, payload: Buffer): Buffer => {
    const body = Buffer.concat([Buffer.from([kind, 2]), payload]), prefix = varint(body.length);
    return Buffer.concat([prefix, body, integer(crc(body), 4), integer(prefix.length + body.length + 8, 4)]);
  };
  const build = (test: any): Buffer[] => {
    const segments: Buffer[] = [];
    for (let index = 0; index < test.segments; index++) {
      const header = Buffer.alloc(32); Buffer.from([137, 83, 80, 82, 13, 10, 26, 10]).copy(header);
      header.writeUInt16LE(1, 8); header.writeUInt32LE(test.mutation === "missing-required-chain" ? 0 : 1, 12); header.writeUInt32LE(crc(header.subarray(0, 20)), 20);
      let bytes = header, chain = digest(header), sequence = 1, previousOffset = 0;
      let pending: Buffer[] = [];
      const append = (kind: number, payload: Buffer): void => { pending.push(frame(kind, payload)); };
      const commit = (): void => {
        chain = digest(Buffer.concat([chain, ...pending.map(digest)]));
        const length = pending.reduce((sum, item) => sum + item.length, 0);
        const payload = Buffer.concat([integer(sequence++, 8), integer(previousOffset, 8), integer(length, 8), integer(pending.length, 4), Buffer.alloc(4), chain]);
        previousOffset = bytes.length + length; bytes = Buffer.concat([bytes, ...pending, frame(12, payload)]); pending = [];
      };
      const document = Buffer.from(index === 1 && test.mutation === "wrong-segment-document" ? "other-document" : proof.documentKey);
      const previous = index ? Buffer.from(segments[index - 1].subarray(-40, -8)) : Buffer.alloc(0);
      if (index && test.mutation === "wrong-prior-tip") previous[0] ^= 1;
      append(64, Buffer.concat([varint(document.length), document, integer(index === 1 && test.mutation === "skipped-segment-index" ? 2 : index, 8), Buffer.from([index ? 1 : 0]), previous])); commit();
      for (const tx of test.segments === 1 ? [1, 2] : [index + 1]) {
        const command = Buffer.from(proof.encodedHex, "hex"); if (tx === 1) command[command.length - 1] ^= 1;
        append(65, integer(tx, 8)); append(68, command); append(66, Buffer.concat([integer(tx, 8), integer(1, 4)])); commit();
      }
      segments.push(bytes);
    }
    if (test.mutation.endsWith("crc-repaired")) {
      const bytes = segments[0], all = frames(bytes);
      const selected = test.mutation === "record-crc-repaired" ? all.find((item) => item.kind === 68)! : all.filter((item) => item.kind === 12)[1];
      const offsets: Record<string, number> = { "commit-hash-crc-repaired": 32, "commit-count-crc-repaired": 24, "commit-length-crc-repaired": 16, "commit-sequence-crc-repaired": 0, "commit-offset-crc-repaired": 8, "commit-reserved-crc-repaired": 28 };
      const offset = test.mutation === "record-crc-repaired" ? selected.end - 1 : test.mutation === "noncritical-commit-crc-repaired" ? selected.body + 1 : selected.body + 2 + offsets[test.mutation];
      if (!Number.isSafeInteger(offset)) throw new Error("unknown oracle mutation");
      bytes[offset] ^= ["record-crc-repaired", "noncritical-commit-crc-repaired"].includes(test.mutation) ? 2 : 1;
      bytes.writeUInt32LE(crc(bytes.subarray(selected.body, selected.end)), selected.end);
    }
    return segments;
  };
  const accepted = (segments: Buffer[], first = 0, requireGenesis = true): boolean => {
    if (requireGenesis && first !== 0) return false;
    let previous: Buffer | undefined;
    for (const [relative, bytes] of segments.slice(first).entries()) {
      const index = relative + first;
      if (bytes.readUInt32LE(12) !== 1 || bytes.readUInt32LE(20) !== crc(bytes.subarray(0, 20))) return false;
      let chain = digest(bytes.subarray(0, 32)), count = 0, length = 0, sequence = 1, previousOffset = 0;
      let pending: Buffer[] = [chain];
      const all = frames(bytes);
      for (const [position, item] of all.entries()) {
        const payload = bytes.subarray(item.body + 2, item.end);
        if (bytes[item.body + 1] !== 2 || bytes.readUInt32LE(item.end) !== crc(bytes.subarray(item.body, item.end))) return false;
        if (item.kind === 12) {
          if (payload.length !== 64 || item.next - item.start !== 75 || payload.readBigUInt64LE(0) !== BigInt(sequence++) || payload.readBigUInt64LE(8) !== BigInt(previousOffset)
            || payload.readBigUInt64LE(16) !== BigInt(length) || payload.readUInt32LE(24) !== count || payload.readUInt32LE(28) !== 0 || !payload.subarray(32).equals(digest(Buffer.concat(pending)))) return false;
          chain = Buffer.from(payload.subarray(32)); pending = [chain]; count = 0; length = 0; previousOffset = item.start;
        } else {
          if (position === 0) {
            const document = Buffer.from(proof.documentKey);
            const tip = previous ?? (index ? payload.subarray(-32) : Buffer.alloc(0));
            const expected = Buffer.concat([varint(document.length), document, integer(index, 8), Buffer.from([index ? 1 : 0]), tip]);
            if (item.kind !== 64 || !payload.equals(expected)) return false;
          } else if (item.kind === 64 || ![65, 66, 68].includes(item.kind)) return false;
          pending.push(digest(bytes.subarray(item.start, item.next))); count++; length += item.next - item.start;
        }
      }
      if (count !== 0 || sequence === 1 || all.at(-1)?.kind !== 12) return false;
      previous = chain;
    }
    return true;
  };
  if (blake3Hex(Buffer.from("abc")) !== "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85") throw new Error("BLAKE3 independent known-answer mismatch");
  for (const test of fixture.cases) {
    const segments = build(test);
    for (const bytes of segments) for (const item of frames(bytes)) if (crc(bytes.subarray(item.body, item.end)) !== bytes.readUInt32LE(item.end)) throw new Error(`oracle hostile CRC was not repaired: ${test.name}`);
    if (accepted(segments) !== test.accepted) throw new Error(`WAL chain oracle mismatch: ${test.name}`);
  }
  for (const owner of fixture.hashingOwnership) {
    if (owner.expected !== (owner.interrupt === "deadline" ? "expired" : "cancelled") || owner.hashingSteps !== 1 || owner.stoppedProgress !== 0 || owner.heldActive !== 1 || owner.releasedActive !== 0) throw new Error("hashing interrupt ownership fixture differs");
  }
  for (const boundary of fixture.retainedBoundaries) {
    const segments = build({ segments: 2, mutation: boundary.mutation });
    if (accepted(segments, 1, false) !== boundary.replayAccepted || accepted(segments, 1, true) !== boundary.genesisProofAccepted) throw new Error("unanchored retained suffix became a genesis proof");
  }
  console.log(`inference-wal-chain-oracle: exact=${fixture.cases.length} hashing-ownership=${fixture.hashingOwnership.length} retained-boundaries=${fixture.retainedBoundaries.length} ajv=1 crc-valid=14 blake3-known-answer=1; Rust replay and third-party blake3 parity pending`);
}

async function proveInferenceCatalogSelectionFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🧪️fixtures", "🎯️inference-catalog-selection-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid catalog selection fixture: ${JSON.stringify(validate.errors)}`);
  for (const test of fixture.cases) {
    const row = structuredClone(fixture);
    if (test.path.length) {
      let at = row;
      for (const key of test.path.slice(0, -1)) at = at[key];
      at[test.path.at(-1)] = test.value;
    }
    const descriptor = row.descriptor, owner = descriptor.owner, selected = row.package;
    const services = row.services.filter((service: any) => service.inferenceSchema === "s.gis.gismap.inference");
    const service = services[0];
    const accepted = row.scope.spaceId === descriptor.spaceId && row.scope.documentId === descriptor.documentId
      && descriptor.artifactKind === "s.gis.gismap" && descriptor.artifactSchema === "gis.map"
      && owner.pluginId === "gis" && owner.packageId === "semio:gis"
      && selected.pluginId === owner.pluginId && selected.packageId === owner.packageId && selected.version === owner.version && selected.componentSha256 === owner.packageHash
      && row.services.length <= 64 && services.length === 1 && service.owner === "gis" && service.contributor === "gis" && service.artifactKind === descriptor.artifactKind
      && service.artifactSchema === "s.gis.gismap" && service.documentSchema === descriptor.artifactSchema && service.dependsOn.length === 0
      && [service.artifactSchemaVersion, service.documentSchemaVersion, service.inferenceSchemaVersion, service.algorithmVersion, service.policyVersion].every((version) => version === 1);
    if (accepted !== test.accepted) throw new Error(`catalog selection mismatch: ${test.name}`);
  }
  console.log(`inference-catalog-projection-oracle: exact=${fixture.cases.length}; no native provider or route authority`);
}

/** 🗺️ Independently pins the whole GIS Map proposal/approval contract without any Rust codec. */
async function proveGisMapProposalApprovalFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub", "🧪️fixtures", "🗳️gis-map-proposal-approval-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ strict: true, allErrors: true });
  ajv.addSchema(JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "🧬️schema", "🔣️.json"), "utf8")));
  const validate = ajv.compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid GIS Map proposal fixture: ${JSON.stringify(validate.errors)}`);
  const hostile = [
    { ...fixture, limits: { ...fixture.limits, proposalMaxBytes: 4097 } },
    { ...fixture, binding: { ...fixture.binding, grantedMode: "read-observe" } },
    { ...fixture, binding: { ...fixture.binding, surfaceId: "s.gis.gismap@1/*#viewer" } },
    { ...fixture, binding: { ...fixture.binding, parentDialect: { ...fixture.binding.parentDialect, subset: "lite" } } },
    { ...fixture, lifecycle: fixture.lifecycle.slice(1) },
    { ...fixture, visibility: fixture.visibility.slice(1) },
    { ...fixture, nonclaims: ["no-external-model-provider", "no-external-model-provider", "no-wgpu-rendering", "no-auto-apply"] },
  ];
  for (const [index, candidate] of hostile.entries()) if (validate(candidate)) throw new Error(`GIS Map proposal fixture accepted hostile mutation ${index}`);
  const hash = (value: string): string => createHash("sha256").update(value, "utf8").digest("hex");
  if (hash(fixture.proposalCanonical) !== fixture.proposalHash || hash(fixture.inverseCanonical) !== fixture.inverseHash) throw new Error("independent proposal/inverse canonical hashes differ");
  const proposal = JSON.parse(fixture.proposalCanonical);
  const inverse = JSON.parse(fixture.inverseCanonical);
  const region = proposal.CreateRegion;
  const regionId = `inference-${fixture.sampleJobId}`;
  if (Object.keys(proposal).length !== 1 || !region || region.index !== fixture.base.snapshot.regions.length || region.item.id !== regionId || region.item.data.id !== regionId || region.item.data.kind !== "inference-bounds") {
    throw new Error("proposal is not exactly one server-stamped CreateRegion for this job");
  }
  if (!Array.isArray(inverse) || inverse.length !== 1 || Object.keys(inverse[0]).length !== 1 || inverse[0].DeleteRegion?.id !== regionId) throw new Error("inverse is not exactly one DeleteRegion for the created id");
  const points: number[][] = [];
  const scan = (value: unknown): void => {
    if (Array.isArray(value)) {
      if (value.length === 2 && value.every((part) => typeof part === "number" && Number.isFinite(part))) points.push(value as number[]);
      else value.forEach(scan);
    } else if (typeof value === "object" && value !== null) {
      const row = value as Record<string, unknown>;
      if (typeof row.lon === "number" && Number.isFinite(row.lon) && typeof row.lat === "number" && Number.isFinite(row.lat)) points.push([row.lon, row.lat]);
      Object.values(row).forEach(scan);
    }
  };
  for (const item of [...fixture.base.snapshot.positions, ...fixture.base.snapshot.routes, ...fixture.base.snapshot.regions]) scan(item.data);
  const fold = points.reduce((box, [lon, lat]) => [Math.min(box[0]!, lon!), Math.max(box[1]!, lon!), Math.min(box[2]!, lat!), Math.max(box[3]!, lat!)], [Infinity, -Infinity, Infinity, -Infinity]);
  const sorted = { lon: [...points.map((point) => point[0]!)].sort((a, b) => a - b), lat: [...points.map((point) => point[1]!)].sort((a, b) => a - b) };
  const bounds = { lonMin: sorted.lon[0]!, lonMax: sorted.lon[sorted.lon.length - 1]!, latMin: sorted.lat[0]!, latMax: sorted.lat[sorted.lat.length - 1]! };
  if (fold[0] !== bounds.lonMin || fold[1] !== bounds.lonMax || fold[2] !== bounds.latMin || fold[3] !== bounds.latMax) throw new Error("two independent bound folds disagree");
  const expected = fixture.base.expectedInference;
  if (JSON.stringify({ positionCount: fixture.base.snapshot.positions.length, routeCount: fixture.base.snapshot.routes.length, regionCount: fixture.base.snapshot.regions.length, bounds }) !== JSON.stringify(expected)) throw new Error("independent counts/bounds differ from the fixture");
  const ring: number[][] = region.item.data.ring;
  const corners = [[bounds.lonMin, bounds.latMin], [bounds.lonMax, bounds.latMin], [bounds.lonMax, bounds.latMax], [bounds.lonMin, bounds.latMax], [bounds.lonMin, bounds.latMin]];
  if (ring.length !== 5 || JSON.stringify(ring) !== JSON.stringify(corners)) throw new Error("proposal ring is not the closed bounds rectangle of the base snapshot");
  if (fixture.preview.schema !== "semio.hub.gis-map-inference-preview/v1"
    || fixture.preview.jobId !== fixture.sampleJobId
    || fixture.preview.proposalHash !== fixture.proposalHash
    || fixture.preview.regionId !== regionId
    || JSON.stringify(fixture.preview.ring) !== JSON.stringify(corners)) {
    throw new Error("owner preview is not the exact bounded projection of the canonical proposal");
  }
  const transitions: Record<string, readonly string[]> = {
    accepted: ["running", "cancel-requested", "cancelled", "failed"],
    running: ["succeeded", "cancel-requested", "cancelled", "failed"],
    succeeded: ["approval-prepared", "cancel-requested", "proposal-cancelled", "proposal-stale"],
    "cancel-requested": ["cancelled", "proposal-cancelled"],
    "approval-prepared": ["approved"],
  };
  for (const trace of [fixture.lifecycle, fixture.cancelLifecycle]) {
    if (trace[0].kind !== "accepted" || trace[0].ordinal !== 1) throw new Error("every private job stream starts at accepted");
    for (let index = 1; index < trace.length; index++) {
      if (trace[index].ordinal !== index + 1) throw new Error("private job event ordinals are not dense");
      if (!(transitions[trace[index - 1].kind] ?? []).includes(trace[index].kind)) throw new Error(`illegal private job transition ${trace[index - 1].kind} -> ${trace[index].kind}`);
    }
    if (trace.length > 6) throw new Error("private job stream exceeds its bounded event ordinal");
  }
  const owners = fixture.visibility.filter((row: { readProposal: boolean }) => row.readProposal);
  if (owners.length !== 1 || owners[0].role !== "author-owner" || !owners[0].approve || owners[0].expectedCode !== null) throw new Error("exactly one original Author owner may read and approve");
  for (const row of fixture.visibility) if (row.role !== "author-owner" && (row.readEvents || row.readProposal || row.approve || row.expectedCode !== "inference.denied")) throw new Error(`non-owner ${row.role} was granted private job access`);
  const statuses = new Map<string, number>();
  for (const row of fixture.errors) {
    if (statuses.has(row.code) && statuses.get(row.code) !== row.status) throw new Error(`error code ${row.code} maps to two statuses`);
    statuses.set(row.code, row.status);
  }
  if (statuses.get("approval.commit-unavailable") !== 503) throw new Error("a missing composition transaction must fail closed with 503");
  for (const rejection of fixture.approvalRejections) if (!statuses.has(rejection.code)) throw new Error(`approval rejection ${rejection.name} uses an unpublished code`);
  const frozen = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️fixtures", "🧊️gis-map-frozen-binding-v1", "🔣️.json"), "utf8"));
  const ledger = JSON.parse(readFileSync(join(repoRoot, "🌎️hub", "🧪️fixtures", "🗺️gis-inference-job-v1", "🔣️.json"), "utf8"));
  if (fixture.binding.digest !== frozen.expectedDigest || fixture.binding.componentBlake3 !== frozen.binding.package.componentBlake3 || fixture.binding.packageVersion !== frozen.binding.package.version
    || JSON.stringify(fixture.binding) !== JSON.stringify(ledger.identity.binding)) throw new Error("proposal, frozen-binding and ledger corpora disagree on the frozen executable identity");
  console.log(`gis-map-proposal-oracle: ajv=1 hostile=${hostile.length} node-sha256=2 independent-bounds=2 preview=1 lifecycle=${fixture.lifecycle.length + fixture.cancelLifecycle.length} visibility=${fixture.visibility.length} errors=${fixture.errors.length} approval-rejections=${fixture.approvalRejections.length} cross-fixture=1; no external model provider, no WGPU rendering`);
  return hostile.length;
}

/** 🧊️ Independently pins every retained GIS Map catalog and executable binding fact. */
async function proveGisMapFrozenBindingFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub", "🧪️fixtures", "🧊️gis-map-frozen-binding-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid frozen GIS Map binding fixture: ${JSON.stringify(validate.errors)}`);
  const digest = (binding: unknown): string => createHash("sha256").update(Buffer.concat([Buffer.from("semio.hub.gis-map-frozen-binding/v1", "utf8"), Buffer.from([0])])).update(JSON.stringify(binding)).digest("hex");
  if (digest(fixture.binding) !== fixture.expectedDigest) throw new Error("frozen GIS Map binding digest differs from the neutral fixture");
  const leafPaths = (value: unknown, path: string[] = []): string[] => value !== null && typeof value === "object" && !Array.isArray(value)
    ? Object.entries(value).flatMap(([key, child]) => leafPaths(child, [...path, key])) : [JSON.stringify(path)];
  const remainingPaths = new Set(leafPaths(fixture.binding));
  const seen = new Set<string>();
  for (const hostile of fixture.hostile) {
    if (seen.has(hostile.name)) throw new Error(`duplicate frozen binding hostile ${hostile.name}`);
    seen.add(hostile.name);
    if (!remainingPaths.delete(JSON.stringify(hostile.path))) throw new Error(`duplicate or unknown frozen binding field: ${hostile.name}`);
    const candidate = structuredClone(fixture.binding);
    let at = candidate;
    for (const key of hostile.path.slice(0, -1)) at = at[key];
    at[hostile.path.at(-1)] = hostile.value;
    const row = { ...fixture, binding: candidate };
    const admitted = validate(row) && digest(candidate) === fixture.expectedDigest;
    if (admitted !== hostile.accepted) throw new Error(`frozen GIS Map binding substitution accepted: ${hostile.name}`);
  }
  if (remainingPaths.size !== 0) throw new Error(`frozen binding fields lack substitution coverage: ${[...remainingPaths].join(", ")}`);
  const catalog = readFileSync(join(repoRoot, "🌎️hub", "💡️inference", "📇️catalog", "🦀️.rs"), "utf8");
  const trusted = readFileSync(join(repoRoot, "🌎️hub", "🗿️artifact-authority", "🔏️trusted-catalog", "🦀️.rs"), "utf8");
  const startup = readFileSync(join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust", "🚀️bin.rs"), "utf8");
  const required = [
    "VerifiedGisMapArtifactBindingV1", "Arc<VerifiedTrustedCatalog>", "selected_document_open", "gis_map_inference_service",
    "executable_identity", "component_blake3", "descriptor_byte_sha256", "parent_dialect", "DocumentOpenSurfaceRoleV1::Editor",
    "grant.read", "grant.write", "grant.observe", "semio.hub.gis-map-frozen-binding/v1\\0",
  ];
  if (required.some((needle) => !catalog.includes(needle)) || !trusted.includes("pub fn selected_document_open")
      || !startup.includes("verified_catalog: Option<Arc<VerifiedTrustedCatalog>>") || !startup.includes("gis_map_binding: Option<Arc<VerifiedGisMapArtifactBindingV1>>")) {
    throw new Error("hub does not retain the exact verified GIS Map catalog and executable binding");
  }
  const checks = fixture.hostile.length + required.length + 5;
  console.log(`gis-map-frozen-binding-check: checks=${checks} clean; no route, provider, inference execution, or publication claim`);
  return checks;
}

class GisInferenceLedgerOracleScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = JSON.parse(readFileSync(join(this.repoRoot, "🌎️hub", "🧪️fixtures", "🗺️gis-inference-job-v1", "🔣️.json"), "utf8"));
    const schemaRoot = join(this.repoRoot, "🌎️hub", "💡️inference", "🧬️schema");
    const schema = JSON.parse(readFileSync(join(schemaRoot, "🔣️.json"), "utf8"));
    const Ajv2020 = (await import("ajv/dist/2020.js")).default;
    const ajv = new Ajv2020({ strict: true, allErrors: true });
    const validate = ajv.compile(schema);
    const validateIdentity = ajv.getSchema(`${schema.$id}#/$defs/identity`);
    const validateFixture = ajv.compile(JSON.parse(readFileSync(join(this.repoRoot, "🌎️hub", "🧪️fixtures", "🗺️gis-inference-job-v1", "🧬️.schema.json"), "utf8")));
    if (!validateFixture(fixture)) throw new Error(`invalid GIS ledger fixture: ${JSON.stringify(validateFixture.errors)}`);
    const { parseInferenceRequestV1 } = await import(join(schemaRoot, "🟦️.ts"));
    if (!validate(fixture.identity.request)) throw new Error("invalid neutral inference intent");
    if (!validateIdentity?.(fixture.identity)) throw new Error(`invalid neutral inference identity: ${JSON.stringify(validateIdentity?.errors)}`);
    const identityRoot = join(this.repoRoot, "🌎️hub/🧪️fixtures/🖥️inference-server-identity-v1");
    const identityFixture = JSON.parse(readFileSync(join(identityRoot, "🔣️.json"), "utf8"));
    if (!ajv.compile(JSON.parse(readFileSync(join(identityRoot, "🧬️.schema.json"), "utf8")))(identityFixture)) throw new Error("invalid server identity corpus");
    for (const row of identityFixture.cases) for (const field of identityFixture.fields) {
      const candidate = { ...fixture.identity, headOrdinal: 1, headEditId: "0".repeat(32), [field]: row.value };
      if (validateIdentity(candidate) !== row.accepted) throw new Error(`server identity parity: ${row.name}/${field}`);
    }
    const maximumId = "a".repeat(identityFixture.maximumBytes);
    const documentKey = `v1:${maximumId.length}:${maximumId.length}:${maximumId}${maximumId}`;
    const actor = `user:${maximumId}#session:${maximumId}`;
    if (Buffer.byteLength(documentKey) > 256 || Buffer.byteLength(actor) > 256) throw new Error("server identifiers exceed the exact command text bound");
    console.log(`inference-server-identity-oracle: cases=${identityFixture.cases.length} fields=${identityFixture.fields.length} composite-bounds=2 ajv+node=1`);
    for (const hostile of fixture.hostileIdentities) {
      const candidate = JSON.parse(JSON.stringify(fixture.identity));
      let at = candidate;
      for (const segment of hostile.path.slice(0, -1)) at = (at[segment] ??= {});
      at[hostile.path[hostile.path.length - 1]] = hostile.value;
      if (validateIdentity(candidate)) throw new Error(`accepted hostile inference identity: ${hostile.name}`);
    }
    parseInferenceRequestV1(fixture.identity.request);
    for (const row of fixture.sqliteIntegers) {
      const integer = BigInt(row.decimal);
      if ((integer >= 0n && integer <= BigInt(Number.MAX_SAFE_INTEGER)) !== row.accepted) throw new Error(`neutral SQLite integer mismatch ${row.decimal}`);
    }
    for (const hostile of fixture.hostileRequests) {
      const candidate = { ...fixture.identity.request, [hostile.field]: hostile.value };
      if (validate(candidate)) throw new Error(`AJV admitted ${hostile.name}`);
      let rejected = false;
      try { parseInferenceRequestV1(candidate); } catch { rejected = true; }
      if (!rejected) throw new Error(`TypeScript admitted ${hostile.name}`);
    }
    const hash = (value: string) => createHash("sha256").update(value).digest("hex");
    if (hash(fixture.input) !== fixture.identity.inputHash || hash(`semio.hub.inference-identity/v1\0${JSON.stringify(fixture.identity)}`) !== fixture.identityDigest) throw new Error("neutral input/identity hash mismatch");
    const outbox = fixture.outbox;
    const jobId = hash(`semio.hub.inference-job-id/v1\0${fixture.identityDigest}`).slice(0, 32);
    if (jobId !== outbox.jobId || hash(outbox.proposal) !== outbox.proposalHash || createHash("sha256").update(Buffer.from(outbox.commandHex, "hex")).digest("hex") !== outbox.commandHash
      || hash(`semio.hub.inference-approval-mutation/v1\0${jobId}\0${outbox.proposalHash}`).slice(0, 32) !== outbox.mutationId) throw new Error("neutral durable outbox identity mismatch");
    const snapshot = JSON.parse(fixture.input);
    const points: number[][] = [];
    const scan = (value: any): void => {
      if (Array.isArray(value)) {
        if (value.length === 2 && value.every((part) => typeof part === "number" && Number.isFinite(part))) points.push(value);
        else value.forEach(scan);
      } else if (typeof value === "object" && value !== null) {
        if (typeof value.lon === "number" && Number.isFinite(value.lon) && typeof value.lat === "number" && Number.isFinite(value.lat)) points.push([value.lon, value.lat]);
        Object.values(value).forEach(scan);
      }
    };
    for (const item of [...snapshot.positions, ...snapshot.routes, ...snapshot.regions]) scan(item.data);
    const bounds = points.length ? { lonMin: Math.min(...points.map((p) => p[0]!)), lonMax: Math.max(...points.map((p) => p[0]!)), latMin: Math.min(...points.map((p) => p[1]!)), latMax: Math.max(...points.map((p) => p[1]!)) } : null;
    const result = { positionCount: snapshot.positions.length, routeCount: snapshot.routes.length, regionCount: snapshot.regions.length, bounds };
    if (JSON.stringify(result) !== JSON.stringify(fixture.expectedInference)) throw new Error("independent deterministic GIS bounds/counts differ");
    for (const trace of fixture.traces) {
      let state = "accepted", proposal = "none", events = 1, hasResult = false;
      for (const operation of trace.operations) {
        if (operation === "start" && state === "accepted") { state = "running"; events++; }
        else if (operation === "succeed" && state === "running") { state = "succeeded"; proposal = "offered"; hasResult = true; events++; }
        else if (operation === "cancel" && (state === "accepted" || state === "running")) { state = "cancelled"; events++; }
        else if (operation === "cancel" && state === "succeeded" && proposal === "offered") { proposal = "cancelled"; hasResult = false; events++; }
      }
      if (state !== trace.state || proposal !== trace.proposalState || events !== trace.eventCount || hasResult !== trace.hasResult) throw new Error(`neutral lifecycle mismatch ${trace.name}`);
    }
    console.log(`gis-inference-ledger-oracle: traces=${fixture.traces.length} hostile=${fixture.hostileRequests.length} identity-hostile=${fixture.hostileIdentities.length} sqlite-integers=${fixture.sqliteIntegers.length} ajv+typescript=1 hashes=6 independent-bounds=1; no executor/route/approval claim`);
    await proveInferenceWalProofFixture(this.repoRoot);
    await proveInferenceCommandFixture(this.repoRoot);
    await proveInferenceApprovalRequestFixture(this.repoRoot);
    await proveInferenceAuthorFixture(this.repoRoot);
    await proveInferenceWalChainFixture(this.repoRoot);
    await proveInferenceCatalogSelectionFixture(this.repoRoot);
    await proveTrustedCatalogIdentityRolesFixture(this.repoRoot);
    await (await import("../../../✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts")).proveGisNativeCodecReceipts(this.repoRoot);
    await (await import("../../../✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts")).proveGisControlledProposal(this.repoRoot);
    await proveGisNativeProviderSelectionFixture(this.repoRoot);
    await proveMemoryBackendBackingFixture(this.repoRoot);
    await proveNativeDeficitFixture(this.repoRoot);
  }
}

/** ✅ Validates the closed approval intent independently of Rust and its transport authority. */
async function proveInferenceApprovalRequestFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🧪️fixtures/✅️inference-approval-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const ajv = new Ajv2020({ strict: true, allErrors: true });
  if (!ajv.compile(schema)(fixture)) throw new Error("invalid inference approval fixture");
  const validate = ajv.getSchema(`${schema.$id}#/$defs/request`)!;
  const decode = (bytes: Buffer): boolean => {
    if (bytes.length > fixture.maximumBytes) return false;
    try { return validate(JSON.parse(bytes.toString("utf8"))) as boolean; } catch { return false; }
  };
  const request = Buffer.from(JSON.stringify(fixture.request));
  if (!decode(request)) throw new Error("valid inference approval intent denied");
  for (const hostile of fixture.hostile) {
    const candidate = { ...fixture.request, [hostile.field]: hostile.value };
    if (decode(Buffer.from(JSON.stringify(candidate)))) throw new Error(`inference approval admitted client authority ${hostile.field}`);
  }
  const boundary = Buffer.alloc(fixture.maximumBytes, 0x20);
  request.copy(boundary);
  if (!decode(boundary) || decode(Buffer.concat([boundary, Buffer.from(" ")]))) throw new Error("inference approval exact byte boundary differs");
  console.log(`inference-approval-request-oracle: valid=1 hostile=${fixture.hostile.length} byte-boundaries=2 ajv+node=1; no approval authority`);
}

/** 🛂 Evaluates durable membership and immutable identity predicates through independent SQLite. */
async function proveInferenceAuthorFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🧪️fixtures/🛂️inference-author-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid inference Author corpus: ${JSON.stringify(validate.errors)}`);
  const { Database } = await import("bun:sqlite");
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE session(user TEXT, session TEXT, generation INTEGER, expires INTEGER, revoked INTEGER); INSERT INTO session VALUES('author','session',1,2000,0); CREATE TABLE membership(space TEXT,user TEXT,role TEXT); INSERT INTO membership VALUES('space','author','author')");
    const predicate = database.query("SELECT EXISTS(SELECT 1 FROM session s JOIN membership m ON m.user=s.user WHERE s.user=?1 AND s.session=?2 AND s.generation=?3 AND s.revoked=0 AND s.expires>?4 AND s.expires>?9 AND ?9>=?4 AND m.space=?5 AND m.role='author' AND ?5='space' AND ?6='document' AND ?4>=0 AND ?7=0 AND ?8=0) AS accepted");
    let accepted = 0;
    const seen = new Set<string>();
    for (const row of fixture.cases) {
      if (seen.has(row.operation)) throw new Error("duplicate inference Author operation");
      seen.add(row.operation);
      database.exec("UPDATE session SET revoked=0; DELETE FROM membership; INSERT INTO membership VALUES('space','author','author')");
      let user = "author", session = "session", generation = 1, now = 1000, returnedAt = 1000, space = "space", document = "document", cancelled = 0, deadline = 0;
      switch (row.operation) {
        case "author": break;
        case "cross-space": space = "other"; break;
        case "cross-document": document = "other"; break;
        case "wrong-user": user = "other"; break;
        case "wrong-session": session = "other"; break;
        case "rotated-generation": generation = 2; break;
        case "spectator": database.exec("UPDATE membership SET role='spectator'"); break;
        case "removed-member": database.exec("DELETE FROM membership"); break;
        case "revoked": database.exec("UPDATE session SET revoked=1"); break;
        case "expiry-exact": now = returnedAt = 2000; break;
        case "expiry-past": now = returnedAt = 2001; break;
        case "expiry-after-read": returnedAt = 2000; break;
        case "clock-regressed": returnedAt = 999; break;
        case "cancelled": cancelled = 1; break;
        case "deadline": deadline = 1; break;
        case "negative-clock": now = -1; break;
        default: throw new Error("unknown inference Author operation");
      }
      const result = predicate.get(user, session, generation, now, space, document, cancelled, deadline, returnedAt) as { accepted: number };
      if ((result.accepted === 1) !== row.accepted) throw new Error(`inference Author predicate mismatch: ${row.operation}`);
      accepted += result.accepted;
    }
    if (accepted !== 1) throw new Error("inference Author corpus must admit exactly one case");
    console.log(`inference-author-oracle: cases=${fixture.cases.length} accepted=${accepted} ajv+sqlite=1; no retained grant or submit authority`);
  } finally { database.close(); }
}

/** 🧷 Independently evaluates the exact GIS selection and admission fences. */
async function proveGisNativeProviderSelectionFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🌍️gis-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid GIS provider selection corpus: ${JSON.stringify(validate.errors)}`);
  const receipts = JSON.parse(readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🔣️.json"), "utf8"));
  if (fixture.packageVersion !== receipts.packageVersion || fixture.codecCount !== receipts.receipts.length) throw new Error("GIS selection differs from the package-owned closure");
  let accepted = 0;
  for (const row of fixture.cases) {
    const result = row.pluginId === receipts.pluginId && row.packageId === receipts.packageId && row.version === receipts.packageVersion && !row.cancelled && row.nowMs < row.deadlineMs;
    if (result !== row.accepted) throw new Error(`GIS native selection mismatch: ${row.name}`);
    if (result) accepted++;
  }
  if (accepted !== 1) throw new Error("GIS provider corpus must admit exactly one selection");
  console.log(`gis-native-provider-selection-oracle: cases=${fixture.cases.length} accepted=${accepted}; no native or catalog activation claim`);
}

/** 🪪️ Keeps descriptor SHA-256 authority distinct from component PackageRef BLAKE3. */
async function proveTrustedCatalogIdentityRolesFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🗿️artifact-authority", "🔏️trusted-catalog", "🧪️fixtures", "🪪️identity-roles");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid catalog identity fixture: ${JSON.stringify(validate.errors)}`);
  const source = JSON.parse(readFileSync(join(root, fixture.source), "utf8"));
  const bytes = Buffer.from(source.componentHex, "hex");
  const sha256 = createHash("sha256").update(bytes).digest("hex");
  if (sha256 !== source.componentSha256 || source.componentSha256 === source.componentBlake3) throw new Error("catalog hash roles lost their independent byte identities");
  const selected = source.bundle.packages[0], target = selected.openTargets[0];
  if (!target.artifactKind.startsWith("s.") || target.surfaceId !== `${target.artifactKind}@1/*#editor` || target.appId !== target.surfaceId) throw new Error("catalog fixture kind/surface is not exact canonical identity");
  for (const test of fixture.cases) {
    let ownerHash = source.componentSha256, kind = target.artifactKind, surface = target.surfaceId;
    switch (test.change) {
      case "blake3-owner": ownerHash = source.componentBlake3; break;
      case "descriptor-owner": ownerHash = selected.descriptor.sha256; break;
      case "zero-owner": ownerHash = "00".repeat(32); break;
      case "bare-kind": kind = kind.slice(2); break;
      case "bare-surface": surface = surface.slice(2); break;
    }
    const codec = ownerHash === sha256 && kind === selected.nativeCodecs[0].artifactKind;
    const open = codec && surface === target.appId;
    if (codec !== test.codec || open !== test.open) throw new Error(`catalog identity role mismatch ${test.change}`);
  }
  console.log(`trusted-catalog-identity-oracle: exact=${fixture.cases.length} canonical-kind=1 descriptor-sha256=1 package-ref-blake3=distinct; no GIS provider activation`);
}

type TrustedBootstrapCodec = { readonly artifactKind: string; readonly artifactSchema: string; readonly packSchemaHash: string };

function trustedBootstrapField(value: Uint8Array | string): Buffer {
  const bytes = typeof value === "string" ? Buffer.from(value, "utf8") : Buffer.from(value);
  const length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(bytes.byteLength));
  return Buffer.concat([length, bytes]);
}

function trustedBootstrapCount(value: number): Buffer {
  const bytes = Buffer.alloc(4);
  bytes.writeUInt32BE(value);
  return bytes;
}

function trustedBootstrapProfileEncoding(profile: any, codecs: Readonly<Record<"gis" | "stdio", readonly TrustedBootstrapCodec[]>>): Buffer {
  const pieces = [Buffer.from("semio/hub/trusted-profile-generation/v1\0"), trustedBootstrapField(profile.id), trustedBootstrapCount(profile.selectedClosure.length)];
  for (const identity of profile.selectedClosure) {
    const selected = profile.packages.find((candidate: any) => candidate.pluginId === identity.pluginId && candidate.packageId === identity.packageId && candidate.version === identity.version);
    if (!selected) throw new Error("trusted bootstrap package is outside selected closure");
    for (const value of [selected.pluginId, selected.packageId, selected.version, selected.role, selected.componentSha256, selected.componentBlake3, selected.descriptorSha256]) {
      pieces.push(trustedBootstrapField(/^[0-9a-f]{64}$/u.test(value) ? Buffer.from(value, "hex") : value));
    }
    pieces.push(trustedBootstrapCount(0));
    const rows = [...codecs[selected.pluginId as "gis" | "stdio"]].sort((left, right) => JSON.stringify([left.artifactKind, left.artifactSchema, left.packSchemaHash]).localeCompare(JSON.stringify([right.artifactKind, right.artifactSchema, right.packSchemaHash])));
    pieces.push(trustedBootstrapCount(rows.length));
    for (const row of rows) pieces.push(trustedBootstrapField(row.artifactKind), trustedBootstrapField(row.artifactSchema), trustedBootstrapField(Buffer.from(row.packSchemaHash, "hex")));
  }
  const targetPackage = profile.packages.find((candidate: any) => candidate.pluginId === profile.openTarget.pluginId);
  const targetCodec = codecs.gis.find((codec) => codec.artifactKind === profile.openTarget.artifactKind && codec.artifactSchema === profile.openTarget.artifactSchema);
  if (!targetPackage || !targetCodec) throw new Error("trusted bootstrap open target has no package codec");
  pieces.push(trustedBootstrapCount(1));
  for (const value of [
    targetPackage.pluginId,
    targetPackage.packageId,
    targetPackage.version,
    Buffer.from(targetPackage.componentSha256, "hex"),
    Buffer.from(targetPackage.componentBlake3, "hex"),
    Buffer.from(targetPackage.descriptorSha256, "hex"),
    profile.openTarget.artifactKind,
    profile.openTarget.artifactSchema,
    Buffer.from(targetCodec.packSchemaHash, "hex"),
    profile.openTarget.parentDialect.artifactKind,
    profile.openTarget.parentDialect.standard,
    profile.openTarget.parentDialect.subset,
    profile.openTarget.surfaceId,
    profile.openTarget.appId,
    profile.openTarget.windowKindId,
    profile.openTarget.role,
    profile.openTarget.rendererTarget,
    Buffer.from([Number(profile.openTarget.grant.read), Number(profile.openTarget.grant.write), Number(profile.openTarget.grant.observe)]),
  ]) pieces.push(trustedBootstrapField(value));
  return Buffer.concat(pieces);
}

function trustedBootstrapClosureEncoding(profile: any): Buffer {
  const pieces = [Buffer.from("semio/hub/trusted-profile-selected-closure/v1\0"), trustedBootstrapCount(profile.selectedClosure.length)];
  for (const identity of profile.selectedClosure) pieces.push(trustedBootstrapField(identity.pluginId), trustedBootstrapField(identity.packageId), trustedBootstrapField(identity.version));
  return Buffer.concat(pieces);
}

function trustedBootstrapPlanGenerationOutcome(issuedGenerationId: string, observedGenerationId: string): "accepted" | "stale" {
  return issuedGenerationId === observedGenerationId ? "accepted" : "stale";
}

/** 🧬️ Independently validates the exact closed stdio+GIS profile and full-generation framing. */
async function proveTrustedStdioGisBootstrapFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid trusted stdio+GIS bootstrap corpus: ${JSON.stringify(validate.errors)}`);
  const stdio = JSON.parse(readFileSync(join(repoRoot, fixture.sources.stdioReceipts), "utf8"));
  const gis = JSON.parse(readFileSync(join(repoRoot, fixture.sources.gisReceipts), "utf8"));
  const codecs: Record<"gis" | "stdio", TrustedBootstrapCodec[]> = {
    stdio: stdio.receipts.map((row: any) => ({ artifactKind: row.artifact_kind, artifactSchema: row.document_schema, packSchemaHash: row.pack_schema_sha256 })),
    gis: gis.receipts.map((row: any) => ({ artifactKind: row.kind, artifactSchema: row.schema, packSchemaHash: row.protocolSha256 })),
  };
  const profile = fixture.profile;
  const unique = (rows: readonly TrustedBootstrapCodec[]): boolean => rows.length === new Set(rows.map((row) => JSON.stringify([row.artifactKind, row.artifactSchema, row.packSchemaHash]))).size && rows.every((row) => /^(?!0{64}$)[0-9a-f]{64}$/u.test(row.packSchemaHash));
  if (stdio.plugin_id !== "stdio" || stdio.package_id !== "semio:stdio" || codecs.stdio.length !== 26 || !unique(codecs.stdio)) throw new Error("stdio bootstrap closure is not exact 26");
  if (gis.pluginId !== "gis" || gis.packageId !== "semio:gis" || gis.packageVersion !== profile.selectedClosure[0].version || codecs.gis.length !== 2 || !unique(codecs.gis) || !codecs.gis.some((row) => row.artifactKind === "s.gis.gismap") || !codecs.gis.some((row) => row.artifactKind === "s.gis.gisterrain")) throw new Error("GIS bootstrap closure is not exact Map plus Terrain");
  if (profile.packages.length !== 2 || profile.packages[0].pluginId !== "gis" || profile.packages[0].codecCount !== 2 || profile.packages[0].targetCount !== 1 || profile.packages[1].pluginId !== "stdio" || profile.packages[1].codecCount !== 26 || profile.packages[1].targetCount !== 0) throw new Error("trusted bootstrap package/target cardinality drifted");
  const closure = trustedBootstrapClosureEncoding(profile);
  const closureNode = createHash("sha256").update(closure).digest("hex");
  const closureWeb = Buffer.from(await crypto.subtle.digest("SHA-256", closure)).toString("hex");
  if (closureNode !== profile.selectedClosureSha256 || closureWeb !== closureNode) throw new Error("trusted bootstrap selected closure digest mismatch");
  const generationBytes = trustedBootstrapProfileEncoding(profile, codecs);
  const generationNode = createHash("sha256").update(generationBytes).digest("hex");
  const generationWeb = Buffer.from(await crypto.subtle.digest("SHA-256", generationBytes)).toString("hex");
  if (generationNode !== profile.generationId || generationWeb !== generationNode) throw new Error(`trusted bootstrap full generation mismatch: ${generationNode}`);
  const changed = structuredClone(profile);
  changed.packages[1].componentSha256 = "31".repeat(32);
  const changedComponent = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  changed.packages[1].componentSha256 = profile.packages[1].componentSha256;
  changed.packages[1].descriptorSha256 = "32".repeat(32);
  const changedDescriptor = createHash("sha256").update(trustedBootstrapProfileEncoding(changed, codecs)).digest("hex");
  const changedCodecs = { ...codecs, stdio: codecs.stdio.map((row, index) => index === 0 ? { ...row, packSchemaHash: "33".repeat(32) } : row) };
  const changedCodec = createHash("sha256").update(trustedBootstrapProfileEncoding(profile, changedCodecs)).digest("hex");
  if ([changedComponent, changedDescriptor, changedCodec].some((digest) => digest === generationNode)) throw new Error("trusted bootstrap generation omits zero-target stdio authority");
  const target = profile.openTarget;
  if (target.pluginId !== "gis" || target.artifactKind !== "s.gis.gismap" || target.surfaceId !== "s.gis.gismap@1/*#editor" || target.appId !== target.surfaceId || target.windowKindId !== "gis2d-main" || target.role !== "editor" || target.rendererTarget !== "wasm" || !target.grant.read || !target.grant.write || !target.grant.observe || codecs.gis.some((row) => row.artifactKind === "s.gis.gisterrain" && row.artifactKind === target.artifactKind)) throw new Error("trusted bootstrap target is not the sole writable GIS Map editor surface");
  const relative = (value: string): boolean => !/^[/\\]|^[A-Za-z]:[/\\]|(?:^|[/\\])\.\.(?:[/\\]|$)/u.test(value);
  if (!relative("packages/gis/component.wasm") || relative("../component.wasm") || relative("C:\\component.wasm")) throw new Error("trusted bootstrap path fence drifted");
  const rotation = fixture.rotation;
  if (fixture.limits.descriptorBytes + 1 !== fixture.limits.descriptorBytesPlusOne || rotation.initialGenerationId !== generationNode || rotation.currentAfterFailedCandidate !== rotation.initialGenerationId || rotation.failedCandidateGenerationId === rotation.initialGenerationId || rotation.nextGenerationId === rotation.initialGenerationId || trustedBootstrapPlanGenerationOutcome(rotation.stalePlan.issuedGenerationId, rotation.stalePlan.observedGenerationId) !== rotation.stalePlan.expected || trustedBootstrapPlanGenerationOutcome(rotation.freshPlan.issuedGenerationId, rotation.freshPlan.observedGenerationId) !== rotation.freshPlan.expected || rotation.stalePlan.issuedGenerationId !== rotation.initialGenerationId || rotation.stalePlan.observedGenerationId !== rotation.nextGenerationId || rotation.freshPlan.issuedGenerationId !== rotation.nextGenerationId || fixture.cancellationStages.length !== 8 || fixture.hostile.length !== 19 || !fixture.hostile.includes("stale-plan-generation")) throw new Error("trusted bootstrap bounds/cancellation/cross-generation rotation corpus drifted");
  const { blake3Hex } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"));
  if (blake3Hex(Buffer.from("abc")) !== "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85") throw new Error("trusted bootstrap first-party BLAKE3 known answer mismatch");
  const descriptor: Record<string, any> = {
    descriptorVersion: 1, packageId: "semio:gis", role: "plugin",
    manifest: { pluginId: "gis", label: "GIS", version: "0.1.0", apps: [], examples: [], capabilities: [], topicContributions: [], commands: [], artifactKinds: [], dependencies: [], contributions: [] },
    activationEvents: [], capabilityRequests: [], extensionPoints: [], execution: "isolated", quotas: {}, contributions: {}, assets: [],
    hashes: { wasmSha256: "41".repeat(32), coreWasmSha256: "42".repeat(32), descriptorSha256: "" },
  };
  descriptor.hashes.descriptorSha256 = createHash("sha256").update(encodePackValue(descriptor)).digest("hex");
  const jsonBytes = Buffer.from(JSON.stringify(descriptor));
  const packBytes = Buffer.from(encodePackValue(descriptor));
  const expected = { pluginId: "gis", packageId: "semio:gis", version: "0.1.0", role: "plugin" as const, execution: "isolated" as const, wasmSha256: descriptor.hashes.wasmSha256, coreWasmSha256: descriptor.hashes.coreWasmSha256 };
  verifyFreshCatalogPackageV1(jsonBytes, packBytes, expected);
  const hostilePairs = [
    [Buffer.from(JSON.stringify({ ...descriptor, manifest: { ...descriptor.manifest, version: "9.9.9" } })), packBytes],
    [jsonBytes, Buffer.concat([packBytes, Buffer.from([0])])],
    [Buffer.from([0xff]), packBytes],
  ] as const;
  for (const [json, pack] of hostilePairs) {
    let rejected = false;
    try { verifyFreshCatalogPackageV1(json, pack, expected); } catch { rejected = true; }
    if (!rejected) throw new Error("fresh catalog package verifier admitted a hostile JSON/pack pair");
  }
  console.log(`trusted-stdio-gis-bootstrap-oracle: packages=2 codecs=${codecs.stdio.length + codecs.gis.length} targets=1 hostile=${fixture.hostile.length} cancellation=${fixture.cancellationStages.length} descriptor-pairs=4 stale-plan=1 ajv+node+webcrypto+first-party-pack+blake3=1; no materialization or hub activation claim`);
}

type TrustedBootstrapMaterializationV1 = Readonly<{ profileId: string; generationId: string; bundleSha256: string; bundlePath: string }>;

function trustedBootstrapWriteNew(path: string, bytes: Uint8Array): void {
  const output = openSync(path, "wx", 0o600);
  let complete = false;
  try {
    let written = 0;
    while (written < bytes.byteLength) written += writeSync(output, bytes, written, bytes.byteLength - written);
    fsyncSync(output);
    complete = true;
  } finally {
    closeSync(output);
    if (!complete) rmSync(path, { force: true });
  }
}

function trustedBootstrapFsyncDirectory(path: string): void {
  if (process.platform === "win32") return;
  const directory = openSync(path, "r");
  try { fsyncSync(directory); } finally { closeSync(directory); }
}

function trustedBootstrapSourceCodecs(repoRoot: string): Record<"gis" | "stdio", TrustedBootstrapCodec[]> {
  const stdio = JSON.parse(readFileSync(join(repoRoot, "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json"), "utf8"));
  const gis = JSON.parse(readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🔣️.json"), "utf8"));
  return {
    stdio: stdio.receipts.map((row: any) => ({ artifactKind: row.artifact_kind, artifactSchema: row.document_schema, packSchemaHash: row.pack_schema_sha256 })),
    gis: gis.receipts.map((row: any) => ({ artifactKind: row.kind, artifactSchema: row.schema, packSchemaHash: row.protocolSha256 })),
  };
}

/** 🏗️ Produces one immutable closed stdio+GIS generation without loading or registering codecs. */
async function materializeTrustedStdioGisBundle(repoRoot: string, dataRoot: string): Promise<TrustedBootstrapMaterializationV1> {
  const trustedRoot = join(dataRoot, "trusted-catalog");
  mkdirSync(trustedRoot, { recursive: true, mode: 0o700 });
  if (lstatSync(trustedRoot).isSymbolicLink() || !lstatSync(trustedRoot).isDirectory()) throw new Error("trusted catalog root must be a regular private directory");
  if (process.platform !== "win32") chmodSync(trustedRoot, 0o700);
  const nonce = randomBytes(16).toString("hex");
  const buildRoot = join(trustedRoot, `build-${nonce}`);
  const stageRoot = join(trustedRoot, `staging-${nonce}`);
  mkdirSync(buildRoot, { mode: 0o700 });
  mkdirSync(stageRoot, { mode: 0o700 });
  let interrupted = false;
  const interrupt = (): void => { interrupted = true; };
  process.on("SIGINT", interrupt);
  process.on("SIGTERM", interrupt);
  const started = Date.now();
  const deadlineMs = 3_600_000;
  const control: FreshBuildControlV1 = {
    cancelled: () => interrupted,
    remainingMs: () => Math.max(0, deadlineMs - (Date.now() - started)),
    checkpoint(stage, completed, total) { console.log(`trusted-stdio-gis-bootstrap ${stage}: ${completed}/${total}`); },
  };
  try {
    const requests = [
      { pluginId: "stdio", cargoPackage: "semio-s-plugin-stdio", componentPackageId: "semio:stdio", outputName: "semio_s_plugin_stdio.wasm", componentProfile: "wasm-release" as const, rootCdylib: true },
      { pluginId: "gis", cargoPackage: "semio-s-plugin-gis", componentPackageId: "semio:gis", outputName: "semio_s_plugin_gis.wasm", componentProfile: "wasm-release" as const, rootCdylib: true },
    ];
    const receipts = new Map<string, FreshComponentReceiptV1>();
    for (const request of requests) {
      const target = join(buildRoot, `${request.pluginId}-target`);
      const stage = join(stageRoot, "packages", request.pluginId);
      mkdirSync(target, { recursive: true, mode: 0o700 });
      mkdirSync(stage, { recursive: true, mode: 0o700 });
      const receipt = await produceFreshComponentV1(repoRoot, request, target, stage, control);
      if (receipt.pluginId !== request.pluginId || receipt.packageId !== request.componentPackageId) throw new Error(`fresh ${request.pluginId} receipt identity changed after production`);
      receipts.set(request.pluginId, receipt);
    }
    const stdio = receipts.get("stdio")!;
    const gis = receipts.get("gis")!;
    const codecs = trustedBootstrapSourceCodecs(repoRoot);
    if (codecs.stdio.length !== 26 || codecs.gis.length !== 2) throw new Error("trusted stdio+GIS codec closure is not exact 26+2");
    const map = codecs.gis.find((codec) => codec.artifactKind === "s.gis.gismap" && codec.artifactSchema === "gis.map");
    const terrain = codecs.gis.find((codec) => codec.artifactKind === "s.gis.gisterrain" && codec.artifactSchema === "gis.terrain");
    if (!map || !terrain) throw new Error("trusted GIS closure must retain exact Map and Terrain codecs");
    const selectedClosure = [
      { pluginId: "gis", packageId: "semio:gis", version: gis.version },
      { pluginId: "stdio", packageId: "semio:stdio", version: stdio.version },
    ];
    const target = {
      artifactKind: "s.gis.gismap",
      artifactSchema: "gis.map",
      packSchemaHash: map.packSchemaHash,
      surfaceId: "s.gis.gismap@1/*#editor",
      appId: "s.gis.gismap@1/*#editor",
      windowKindId: "gis2d-main",
      role: "editor",
      rendererTarget: "wasm",
      parentDialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "*" },
      grant: { read: true, write: true, observe: true },
    };
    const packageSummary = [
      { pluginId: "gis", packageId: "semio:gis", version: gis.version, role: "plugin", componentSha256: gis.component.sha256, componentBlake3: gis.component.blake3, descriptorSha256: gis.descriptor.sha256, codecCount: 2, targetCount: 1 },
      { pluginId: "stdio", packageId: "semio:stdio", version: stdio.version, role: "plugin", componentSha256: stdio.component.sha256, componentBlake3: stdio.component.blake3, descriptorSha256: stdio.descriptor.sha256, codecCount: 26, targetCount: 0 },
    ];
    const profileSummary = { id: "local-stdio-gis-open-v1", selectedClosure, packages: packageSummary, openTarget: { pluginId: "gis", ...target } };
    const selectedClosureSha256 = createHash("sha256").update(trustedBootstrapClosureEncoding(profileSummary)).digest("hex");
    const generationId = createHash("sha256").update(trustedBootstrapProfileEncoding(profileSummary, codecs)).digest("hex");
    const file = (plugin: "gis" | "stdio", receipt: FreshComponentReceiptV1) => ({
      pluginId: plugin,
      packageId: receipt.packageId,
      version: receipt.version,
      role: "plugin",
      dependencies: [],
      component: { path: `packages/${plugin}/component.wasm`, byteLength: receipt.component.byteLength, sha256: receipt.component.sha256, blake3: receipt.component.blake3 },
      descriptor: { path: `packages/${plugin}/descriptor.semio`, byteLength: receipt.descriptor.byteLength, sha256: receipt.descriptor.sha256 },
      nativeCodecs: codecs[plugin],
      openTargets: plugin === "gis" ? [target] : [],
    });
    const bundle = {
      schemaVersion: 2,
      profiles: [{ id: profileSummary.id, selectedClosure, selectedClosureSha256, openTarget: { package: selectedClosure[0], target }, generationId }],
      packages: [file("gis", gis), file("stdio", stdio)],
    };
    const bundleBytes = Buffer.from(`${JSON.stringify(bundle)}\n`, "utf8");
    if (bundleBytes.byteLength > 4 * 1024 * 1024) throw new Error("trusted stdio+GIS bundle exceeds 4 MiB");
    const bundlePath = join(stageRoot, "trusted-catalog.json");
    trustedBootstrapWriteNew(bundlePath, bundleBytes);
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages", "stdio"));
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages", "gis"));
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(stageRoot);
    if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted stdio+GIS bootstrap cancelled before publication");
    const generations = join(trustedRoot, "generations");
    mkdirSync(generations, { mode: 0o700 });
    const generationRoot = join(generations, generationId);
    if (existsSync(generationRoot)) {
      const existingBundle = join(generationRoot, "trusted-catalog.json");
      if (!lstatSync(generationRoot).isDirectory() || !existsSync(existingBundle) || !readFileSync(existingBundle).equals(bundleBytes)) throw new Error("existing trusted generation differs from the exact immutable bundle");
      for (const [plugin, receipt] of [["gis", gis], ["stdio", stdio]] as const) {
        const component = readFileSync(join(generationRoot, "packages", plugin, "component.wasm"));
        const descriptor = readFileSync(join(generationRoot, "packages", plugin, "descriptor.semio"));
        if (component.byteLength !== receipt.component.byteLength || descriptor.byteLength !== receipt.descriptor.byteLength || createHash("sha256").update(component).digest("hex") !== receipt.component.sha256 || createHash("sha256").update(descriptor).digest("hex") !== receipt.descriptor.sha256) throw new Error("existing trusted generation artifact differs from its immutable receipt");
      }
      rmSync(stageRoot, { recursive: true, force: true });
    } else {
      renameSync(stageRoot, generationRoot);
      trustedBootstrapFsyncDirectory(generations);
    }
    const bundleSha256 = createHash("sha256").update(bundleBytes).digest("hex");
    return { profileId: profileSummary.id, generationId, bundleSha256, bundlePath: join(generationRoot, "trusted-catalog.json") };
  } catch (error) {
    rmSync(stageRoot, { recursive: true, force: true });
    throw error;
  } finally {
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", interrupt);
    rmSync(buildRoot, { recursive: true, force: true });
  }
}

function trustedBootstrapReadRegular(path: string, maximum: number, label: string): Buffer {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile() || info.size === 0 || info.size > maximum) throw new Error(`${label} is not a bounded regular file`);
  const bytes = readFileSync(path);
  if (bytes.byteLength !== info.size) throw new Error(`${label} changed during its bounded read`);
  return bytes;
}

/** 🔄️ Reissues the zero-target Stdio descriptor as a distinct immutable, fully verified generation. */
async function materializeTrustedStdioGisRotation(repoRoot: string, dataRoot: string, current: TrustedBootstrapMaterializationV1): Promise<TrustedBootstrapMaterializationV1> {
  const retained = trustedBootstrapCurrent(dataRoot);
  if (!retained || JSON.stringify(retained) !== JSON.stringify(current)) throw new Error("trusted stdio+GIS rotation did not start from the retained current generation");
  const sourceBundleBytes = trustedBootstrapReadRegular(current.bundlePath, 4 * 1024 * 1024, "trusted rotation source bundle");
  const bundle = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(sourceBundleBytes)) as Record<string, any>;
  if (bundle.schemaVersion !== 2 || bundle.profiles?.length !== 1 || bundle.packages?.length !== 2) throw new Error("trusted rotation source is not the exact closed profile");
  const profile = bundle.profiles[0]!;
  const trustedRoot = join(dataRoot, "trusted-catalog");
  const generationsRoot = join(trustedRoot, "generations");
  const stageRoot = join(trustedRoot, `rotation-${randomBytes(16).toString("hex")}`);
  mkdirSync(stageRoot, { mode: 0o700 });
  try {
    const codecs = {} as Record<"gis" | "stdio", TrustedBootstrapCodec[]>;
    for (const plugin of ["gis", "stdio"] as const) {
      const record = bundle.packages.find((candidate: any) => candidate.pluginId === plugin);
      if (!record || record.packageId !== `semio:${plugin}` || record.role !== "plugin" || !Array.isArray(record.nativeCodecs)) throw new Error(`trusted rotation lost ${plugin} package authority`);
      codecs[plugin] = record.nativeCodecs;
      const sourceRoot = join(generationsRoot, current.generationId, "packages", plugin);
      const destinationRoot = join(stageRoot, "packages", plugin);
      mkdirSync(destinationRoot, { recursive: true, mode: 0o700 });
      const component = trustedBootstrapReadRegular(join(sourceRoot, "component.wasm"), 64 * 1024 * 1024, `${plugin} rotation component`);
      let descriptor = trustedBootstrapReadRegular(join(sourceRoot, "descriptor.semio"), 4 * 1024 * 1024, `${plugin} rotation descriptor`);
      if (component.byteLength !== record.component.byteLength || createHash("sha256").update(component).digest("hex") !== record.component.sha256 || descriptor.byteLength !== record.descriptor.byteLength || createHash("sha256").update(descriptor).digest("hex") !== record.descriptor.sha256) throw new Error(`trusted rotation ${plugin} source differs from its retained receipt`);
      if (plugin === "stdio") {
        const value = decodePackValue(descriptor) as Record<string, any>;
        if (value.packageId !== record.packageId || value.manifest?.pluginId !== plugin || value.manifest?.version !== record.version || value.role !== record.role || value.execution !== "isolated" || typeof value.hashes?.coreWasmSha256 !== "string") throw new Error("trusted rotation Stdio descriptor identity changed");
        value.manifest.label = `Stdio trusted rotation ${randomBytes(8).toString("hex")}`;
        value.hashes.descriptorSha256 = "";
        value.hashes.descriptorSha256 = createHash("sha256").update(encodePackValue(value)).digest("hex");
        descriptor = Buffer.from(encodePackValue(value));
        const json = Buffer.from(JSON.stringify(value), "utf8");
        try {
          verifyFreshCatalogPackageV1(json, descriptor, { pluginId: plugin, packageId: record.packageId, version: record.version, role: "plugin", execution: "isolated", wasmSha256: record.component.sha256, coreWasmSha256: value.hashes.coreWasmSha256 });
        } finally { json.fill(0); }
        record.descriptor.byteLength = descriptor.byteLength;
        record.descriptor.sha256 = createHash("sha256").update(descriptor).digest("hex");
      }
      trustedBootstrapWriteNew(join(destinationRoot, "component.wasm"), component);
      trustedBootstrapWriteNew(join(destinationRoot, "descriptor.semio"), descriptor);
      trustedBootstrapFsyncDirectory(destinationRoot);
    }
    const packageSummary = bundle.packages.map((record: any) => ({
      pluginId: record.pluginId, packageId: record.packageId, version: record.version, role: record.role,
      componentSha256: record.component.sha256, componentBlake3: record.component.blake3, descriptorSha256: record.descriptor.sha256,
      codecCount: record.nativeCodecs.length, targetCount: record.openTargets.length,
    }));
    const profileSummary = { id: profile.id, selectedClosure: profile.selectedClosure, packages: packageSummary, openTarget: { pluginId: profile.openTarget.package.pluginId, ...profile.openTarget.target } };
    const generationId = createHash("sha256").update(trustedBootstrapProfileEncoding(profileSummary, codecs)).digest("hex");
    if (generationId === current.generationId) throw new Error("trusted rotation did not change the full profile generation");
    profile.generationId = generationId;
    const bundleBytes = Buffer.from(`${JSON.stringify(bundle)}\n`, "utf8");
    if (bundleBytes.byteLength > 4 * 1024 * 1024) throw new Error("trusted rotation bundle exceeds 4 MiB");
    trustedBootstrapWriteNew(join(stageRoot, "trusted-catalog.json"), bundleBytes);
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(stageRoot);
    const generationRoot = join(generationsRoot, generationId);
    if (existsSync(generationRoot)) throw new Error("trusted rotation generation already exists");
    renameSync(stageRoot, generationRoot);
    trustedBootstrapFsyncDirectory(generationsRoot);
    return { profileId: profile.id, generationId, bundleSha256: createHash("sha256").update(bundleBytes).digest("hex"), bundlePath: join(generationRoot, "trusted-catalog.json") };
  } catch (error) {
    rmSync(stageRoot, { recursive: true, force: true });
    throw error;
  }
}

function trustedBootstrapCurrent(dataRoot: string): TrustedBootstrapMaterializationV1 | undefined {
  const trustedRoot = join(dataRoot, "trusted-catalog");
  const pointerPath = join(trustedRoot, "current.json");
  if (!existsSync(pointerPath)) return undefined;
  const info = lstatSync(pointerPath);
  if (info.isSymbolicLink() || !info.isFile() || info.size === 0 || info.size > 64 * 1024) throw new Error("trusted catalog current pointer is not a bounded regular file");
  const bytes = readFileSync(pointerPath);
  let pointer: any;
  try { pointer = JSON.parse(bytes.toString("utf8")); } catch { throw new Error("trusted catalog current pointer does not decode"); }
  if (JSON.stringify(Object.keys(pointer).sort()) !== JSON.stringify(["bundleSha256", "generationId", "profileId"]) || pointer.profileId !== "local-stdio-gis-open-v1" || !/^[0-9a-f]{64}$/u.test(pointer.generationId) || !/^[0-9a-f]{64}$/u.test(pointer.bundleSha256) || !bytes.equals(Buffer.from(`${JSON.stringify(pointer)}\n`, "utf8"))) throw new Error("trusted catalog current pointer is not exact canonical metadata");
  const bundlePath = join(trustedRoot, "generations", pointer.generationId, "trusted-catalog.json");
  const bundleInfo = lstatSync(bundlePath);
  if (bundleInfo.isSymbolicLink() || !bundleInfo.isFile() || bundleInfo.size === 0 || bundleInfo.size > 4 * 1024 * 1024) throw new Error("trusted catalog current bundle is not a bounded regular file");
  const bundleBytes = readFileSync(bundlePath);
  const bundle = JSON.parse(bundleBytes.toString("utf8"));
  if (createHash("sha256").update(bundleBytes).digest("hex") !== pointer.bundleSha256 || bundle.schemaVersion !== 2 || bundle.profiles?.length !== 1 || bundle.profiles[0]?.id !== pointer.profileId || bundle.profiles[0]?.generationId !== pointer.generationId) throw new Error("trusted catalog current pointer differs from its immutable bundle");
  return { profileId: pointer.profileId, generationId: pointer.generationId, bundleSha256: pointer.bundleSha256, bundlePath };
}

function publishTrustedBootstrapCurrent(dataRoot: string, receipt: TrustedBootstrapMaterializationV1): void {
  const pointer = Buffer.from(`${JSON.stringify({ profileId: receipt.profileId, generationId: receipt.generationId, bundleSha256: receipt.bundleSha256 })}\n`, "utf8");
  const trustedRoot = join(dataRoot, "trusted-catalog");
  const temporary = join(trustedRoot, `.current-${randomBytes(16).toString("hex")}.json`);
  try {
    trustedBootstrapWriteNew(temporary, pointer);
    renameSync(temporary, join(trustedRoot, "current.json"));
    trustedBootstrapFsyncDirectory(trustedRoot);
  } finally { rmSync(temporary, { force: true }); }
}

async function proveTrustedStdioGisCandidatePlan(run: LocalHubRun, receipt: TrustedBootstrapMaterializationV1, envelope: Record<string, any>): Promise<Record<string, any>> {
  const bundle = JSON.parse(readFileSync(receipt.bundlePath, "utf8"));
  const profile = bundle.profiles?.find((candidate: any) => candidate.id === receipt.profileId);
  const selected = bundle.packages?.find((candidate: any) => candidate.pluginId === "gis");
  const target = profile?.openTarget?.target;
  if (!profile || !selected || !target) throw new Error("trusted stdio+GIS candidate bundle lost its exact GIS target");
  const headers = { authorization: `Bearer ${envelope.capability}`, "content-type": "application/json" };
  const created = await postLiveDirectoryCommand(run, envelope.capability, liveDirectoryCommandRequestId(), { kind: "create-space", name: "Trusted GIS Bootstrap Probe", spaceKind: "studio", visibility: "private" });
  const createdBody = created.status === 202 ? JSON.parse(created.text) as Record<string, any> : undefined;
  const spaceId = createdBody?.events?.find((candidate: any) => candidate?.body?.kind === "space.created")?.body?.spaceId;
  if (typeof spaceId !== "string" || spaceId.length === 0) throw new Error("trusted stdio+GIS candidate could not create its private probe space");
  const documentId = `trusted-gis-map-${randomBytes(8).toString("hex")}`;
  const descriptor = {
    spaceId, documentId, artifactKind: target.artifactKind, artifactSchema: target.artifactSchema,
    owner: { pluginId: selected.pluginId, packageId: selected.packageId, version: selected.version, packageHash: selected.component.sha256 },
    packSchemaHash: target.packSchemaHash, bootstrapVersion: 1,
    bootstrapFrontier: { headSeq: 0, commitSeq: 0, epoch: 0 }, bootstrapSnapshotHash: "11".repeat(32),
  };
  const announced = await postLiveDirectoryCommand(run, envelope.capability, liveDirectoryCommandRequestId(), { kind: "announce-document", descriptor });
  if (announced.status !== 202) throw new Error(`trusted stdio+GIS candidate could not announce its GIS Map probe: ${announced.status}`);
  const response = await fetch(`http://127.0.0.1:${run.port}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
    method: "POST", headers, signal: AbortSignal.timeout(2_000),
    body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: target.surfaceId, clientInstanceId: "trusted-bootstrap-candidate" }),
  });
  const plan = parseDocumentOpenPlanV1(await response.json().catch(() => undefined));
  if (!response.ok || plan.scope.spaceId !== spaceId || plan.scope.documentId !== documentId || plan.catalog.generationId !== receipt.generationId
    || JSON.stringify(plan.package) !== JSON.stringify({ pluginId: selected.pluginId, packageId: selected.packageId, version: selected.version, componentSha256: selected.component.sha256, componentBlake3: selected.component.blake3, descriptorByteSha256: selected.descriptor.sha256 })
    || JSON.stringify(plan.artifact) !== JSON.stringify({ kind: target.artifactKind, schema: target.artifactSchema, packSchemaHash: target.packSchemaHash })
    || JSON.stringify(plan.parentDialect) !== JSON.stringify(target.parentDialect) || JSON.stringify(plan.surface) !== JSON.stringify({ surfaceId: target.surfaceId, appId: target.appId, windowKindId: target.windowKindId, role: target.role, rendererTarget: target.rendererTarget })
    || JSON.stringify(plan.grant) !== JSON.stringify(target.grant)) throw new Error("trusted stdio+GIS candidate issued a substituted GIS Map plan");
  return plan;
}

async function proveTrustedStdioGisStalePlanRejected(run: LocalHubRun, stalePlan: Record<string, any>, envelope: Record<string, any>): Promise<void> {
  try {
    const response = await fetch(`http://127.0.0.1:${run.port}/spaces/${encodeURIComponent(stalePlan.scope.spaceId)}/documents/${encodeURIComponent(stalePlan.scope.documentId)}/socket-grants`, {
      method: "POST",
      headers: { authorization: `Bearer ${envelope.capability}`, "content-type": "application/json" },
      signal: AbortSignal.timeout(2_000),
      body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: stalePlan.receipt }),
    });
    const body = await response.json().catch(() => undefined) as Record<string, any> | undefined;
    if (response.status !== 401 || JSON.stringify(body) !== JSON.stringify({ schema: "semio.hub.document-open-plan-error/v1", code: "denied" })) throw new Error("prior-generation authenticated plan was not terminally denied before fresh issuance");
  } finally { stalePlan.receipt = ""; }
}

/** 🟢️ Publishes current metadata only after one isolated candidate loads the immutable generation. */
async function validateAndPublishTrustedStdioGisCandidate(repoRoot: string, hubRoot: string, dataRoot: string, receipt: TrustedBootstrapMaterializationV1, binaryPath?: string, stalePlan?: Record<string, any>): Promise<Record<string, any>> {
  const profile: LocalProfile = { profileId: "trusted-bootstrap-probe", subject: "trusted-bootstrap-subject", displayName: "Trusted Bootstrap Probe", allowedClientClasses: ["native"] };
  const candidate = await startLocalHub(repoRoot, hubRoot, [profile], { capture: true, dataDir: join(dataRoot, "candidate-data"), trustedCatalog: receipt, binaryPath });
  let envelope: Record<string, any> | undefined;
  try {
    const readiness = await waitForReadiness(candidate);
    if (readiness.artifactAuthority?.ready !== true || readiness.features?.openPlan !== true || readiness.features?.openPlanExchange !== true) throw new Error("trusted stdio+GIS candidate did not expose one verified document-open authority");
    envelope = await issueLocalCredential(candidate, profile.profileId, "native");
    if (stalePlan) await proveTrustedStdioGisStalePlanRejected(candidate, stalePlan, envelope);
    const plan = await proveTrustedStdioGisCandidatePlan(candidate, receipt, envelope);
    publishTrustedBootstrapCurrent(dataRoot, receipt);
    return plan;
  } finally {
    if (envelope) envelope.capability = "";
    await finishLocalHub(candidate);
  }
}

/** ✉️ Independent bounded canonical envelope oracle; it does not interpret GIS mutations. */
async function proveInferenceCommandFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub", "🧪️fixtures", "✉️inference-command-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`invalid inference command fixture: ${JSON.stringify(validate.errors)}`);
  const source = JSON.parse(readFileSync(join(root, fixture.source), "utf8"));
  const limits = fixture.limits;
  const variable = (value: number | bigint): Buffer => {
    let remaining = BigInt(value); const bytes: number[] = [];
    do { const byte = Number(remaining & 127n); remaining >>= 7n; bytes.push(byte | (remaining ? 128 : 0)); } while (remaining);
    return Buffer.from(bytes);
  };
  const encode = (command: any): Buffer => {
    const parts: Buffer[] = [];
    const field = (value: Buffer): void => { parts.push(variable(value.length), value); };
    const text = (value: string): void => field(Buffer.from(value, "utf8"));
    text(command.mutationId); text(command.documentId); text(command.actor);
    parts.push(variable(command.dependencies.length)); command.dependencies.forEach(text);
    text(command.diff.schema); field(Buffer.from(command.diff.payloadHex, "hex"));
    text(command.inverse.schema); field(Buffer.from(command.inverse.payloadHex, "hex"));
    parts.push(variable(command.timestamp.actor), variable(command.timestamp.physicalMs), variable(command.timestamp.logical));
    return Buffer.concat(parts);
  };
  const decode = (bytes: Buffer): any => {
    if (!bytes.length || bytes.length > limits.commandBytes) throw new Error("bounds");
    let position = 0;
    const integer = (): number => {
      const start = position; let value = 0n;
      for (let shift = 0n; shift <= 63n; shift += 7n) {
        if (position === bytes.length) throw new Error("truncated");
        const byte = bytes[position++];
        if (shift === 63n && byte > 1) throw new Error("overflow");
        value |= BigInt(byte & 127) << shift;
        if (!(byte & 128)) {
          if (value > BigInt(limits.integerMaximum) || !variable(value).equals(bytes.subarray(start, position))) throw new Error("noncanonical");
          return Number(value);
        }
      }
      throw new Error("overflow");
    };
    const field = (maximum: number): Buffer => {
      const length = integer();
      if (length > maximum || length > bytes.length - position) throw new Error("bounds");
      const value = bytes.subarray(position, position + length); position += length; return value;
    };
    const text = (): string => {
      const value = new TextDecoder("utf-8", { fatal: true }).decode(field(limits.textBytes));
      if (!value || /\p{Cc}/u.test(value)) throw new Error("text");
      return value;
    };
    const command: any = { mutationId: text(), documentId: text(), actor: text(), dependencies: [] };
    const count = integer();
    if (count > limits.dependencyCount) throw new Error("bounds");
    for (let index = 0; index < count; index++) {
      const value = text(); if (command.dependencies.includes(value)) throw new Error("duplicate"); command.dependencies.push(value);
    }
    command.diff = { schema: text(), payloadHex: field(limits.payloadBytes).toString("hex") };
    command.inverse = { schema: text(), payloadHex: field(limits.payloadBytes).toString("hex") };
    command.timestamp = { actor: integer(), physicalMs: integer(), logical: integer() };
    if (position !== bytes.length || !encode(command).equals(bytes)) throw new Error("trailing");
    return command;
  };
  const canonical = encode(source.command);
  if (canonical.toString("hex") !== source.encodedHex || createHash("sha256").update(canonical).digest("hex") !== source.commandHash) throw new Error("canonical source mismatch");
  const webDigest = Buffer.from(await crypto.subtle.digest("SHA-256", canonical)).toString("hex");
  if (webDigest !== source.commandHash) throw new Error("independent WebCrypto command hash mismatch");
  for (const vector of fixture.vectors) {
    const command = structuredClone(source.command);
    switch (vector.change) {
      case "text-max-plus-one": command.actor = "a".repeat(limits.textBytes + 1); break;
      case "control-text": command.actor += "\u0001"; break;
      case "empty-schema": command.diff.schema = ""; break;
      case "dependency-max-plus-one": command.dependencies = Array.from({ length: limits.dependencyCount + 1 }, (_, index) => index.toString(16).padStart(32, "0")); break;
      case "duplicate-dependency": command.dependencies = ["e".repeat(32), "e".repeat(32)]; break;
      case "diff-max-plus-one": command.diff.payloadHex = "00".repeat(limits.payloadBytes + 1); break;
      case "inverse-max-plus-one": command.inverse.payloadHex = "00".repeat(limits.payloadBytes + 1); break;
      case "hlc-actor-max-plus-one": command.timestamp.actor = BigInt(limits.integerMaximum) + 1n; break;
      case "hlc-time-max-plus-one": command.timestamp.physicalMs = BigInt(limits.integerMaximum) + 1n; break;
      case "hlc-logical-max-plus-one": command.timestamp.logical = BigInt(limits.integerMaximum) + 1n; break;
      case "different-actor": command.actor = command.actor.replace("a", "e"); break;
      case "different-scope": command.documentId = command.documentId.replace("c", "e"); break;
      case "different-mutation": command.mutationId = "e".repeat(32); break;
    }
    let bytes = encode(command);
    switch (vector.change) {
      case "trailing": bytes = Buffer.concat([bytes, Buffer.from([0])]); break;
      case "truncated": bytes = bytes.subarray(0, -1); break;
      case "oversize": bytes = Buffer.alloc(limits.commandBytes + 1); break;
      case "overlong-varint": bytes = Buffer.concat([Buffer.from([bytes[0] | 128, 0]), bytes.subarray(1)]); break;
      case "overflow-varint": bytes = Buffer.concat([Buffer.alloc(9, 255), Buffer.from([2]), bytes.subarray(1)]); break;
      case "invalid-utf8": bytes[1] = 255; break;
    }
    let outcome = "rejected";
    try {
      const value = decode(bytes);
      outcome = ["mutationId", "documentId", "actor"].every(key => value[key] === source.command[key]) ? "canonical" : "identity-denied";
    } catch {}
    if (outcome !== vector.expected) throw new Error(`inference command boundary mismatch: ${vector.name}`);
  }
  console.log(`inference-command-oracle: vectors=${fixture.vectors.length} ajv=1 node+webcrypto-hash=2; no GIS execution authority`);
}

async function proveMemoryBackendBackingFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️fixtures/🧮️memory-backing");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`memory backing schema: ${JSON.stringify(validate.errors)}`);
  const hostile = [
    { ...fixture, maximumInlineBytes: fixture.maximumInlineBytes + 1 },
    { ...fixture, tables: fixture.tables.slice(1) },
    { ...fixture, tables: [{ ...fixture.tables[0], slots: 65 }, ...fixture.tables.slice(1)] },
    { ...fixture, retry: { ...fixture.retry, timerDelayMs: 0 } },
    { ...fixture, sequentialTasks: 65 },
  ];
  if (hostile.some(value => validate(value))) throw new Error("memory backing schema accepted altered bounds");
  const inline = 128;
  const lengths = fixture.tables.map((row: { slots: number }, index: number) => row.slots * (index + 1) * 8);
  const required = BigInt(inline) + lengths.reduce((sum: bigint, length: number) => sum + BigInt(length), 0n);
  for (const test of fixture.admission) {
    const remaining = test.remaining === "exact" ? required : test.remaining === "one-short" ? required - 1n : 0n;
    const accepted = required <= remaining;
    if (accepted !== test.accepted) throw new Error(`memory backing admission differs: ${test.remaining}`);
    if (!accepted) continue;
    const tables = lengths.map((length: number) => Buffer.alloc(length));
    const actual = BigInt(inline) + tables.reduce((sum: bigint, bytes: Buffer) => sum + BigInt(bytes.byteLength), 0n);
    if (actual !== required) throw new Error("memory backing allocation differs from reserved bytes");
    tables.length = 0;
  }
  const pendingTasks = new Set([1]);
  if ((pendingTasks.size === 0) !== fixture.closeWhileAdmitted) throw new Error("memory backing retired an admitted task");
  pendingTasks.delete(1);
  if (pendingTasks.size !== 0) throw new Error("memory backing task admission was not returned");
  const heldResult = Buffer.from([0]);
  for (let index = 0; index < fixture.sequentialTasks; index++) {
    pendingTasks.add(index + 1);
    await Promise.resolve();
    pendingTasks.delete(index + 1);
    if (pendingTasks.size !== 0 || heldResult[0] !== 0) throw new Error("sequential task retirement changed a retained result or leaked task admission");
  }
  let queueOccupied = true, retryAttempts = 0, terminal = false;
  const retry = async (): Promise<void> => {
    while (!terminal && retryAttempts < fixture.retry.maximumAttempts) {
      retryAttempts++;
      await new Promise<void>(resolve => setTimeout(resolve, fixture.retry.timerDelayMs));
      terminal = !queueOccupied;
    }
  };
  const pendingRetry = retry();
  queueMicrotask(() => { queueOccupied = false; });
  await pendingRetry;
  if (terminal !== fixture.retry.terminalAfterQueueRelease || retryAttempts !== 1) throw new Error("memory backing retry did not reach terminal after queue release");
  console.log(`memory-backing-oracle: tables=${fixture.tables.length} admission=${fixture.admission.length} hostile=${hostile.length} timer-retry=1 sequential=${fixture.sequentialTasks}; runtime ABI sizes and worker wake are checked by the Rust owner laws`);
}

async function proveNativeDeficitFixture(repoRoot: string): Promise<void> {
  const { toolJobCooperativeMaintenanceSelfTests } = await import(join(repoRoot, "📜️script.ts"));
  const checks = toolJobCooperativeMaintenanceSelfTests();
  if (checks !== 8) throw new Error("deficit oracle must execute all six lanes and two hostile bounds");
  console.log(`native-deficit-oracle: lanes=6 maximum-rounds=8 checks=${checks}; shared independent oracle, cooperative host turns remain distinct`);
}

/** 🪶️ Verifies the SQLite foundation without representing it as the later two-user route gate. */
class GisInferenceLedgerCheckScript extends BundleScript {
  async run(): Promise<void> {
    runCmd("bun", ["./📜️script.ts", "gis-inference-ledger-oracle"], { cwd: this.root, budgetMs: 60_000 });
    const suffixes = [
      "gis_inference_sqlite_ledger_executes_neutral_traces_with_private_first_terminal_wins",
      "gis_inference_sqlite_request_identity_capacity_expiry_and_progress_are_bounded",
      "gis_inference_sqlite_concurrent_connections_have_one_durable_request_winner",
      "gis_inference_sqlite_prepared_approval_survives_restart_and_reconciles_exactly_once",
      "inference_wal_proof_executes_literal_committed_transaction_scope_and_cancellation_traces",
      "inference_wal_proof_dropped_caller_cancels_and_finishes_retained_replay_before_release",
      "inference_wal_chain_rejects_crc_valid_tampering_and_exact_cross_segment_tip_mismatch",
      "inference_wal_chain_cancellation_retires_hashing_and_compacted_suffix_is_not_a_genesis_proof",
      "inference_catalog_projection_requires_exact_scope_package_and_declared_service",
      "inference_command_exact_decoder_executes_neutral_bounds_canonical_eof_and_actor_vectors",
      "inference_wal_proof_rejects_hash_matched_noncanonical_or_wrong_actor_commands",
      "loader_retains_exact_bytes_and_independent_identities_before_atomic_codec_activation",
      "verified_trusted_catalog_document_open_generation_and_resolution_are_exact",
      "inference_approval_request_accepts_only_job_digest_and_exact_body_bound",
      "inference_live_author_rechecks_real_sqlite_session_scope_role_revocation_and_cancellation",
      "gis_native_provider_selection_binds_literal_owner_version_and_cancellation_without_publication",
    ];
    const receipts = await runExactCargoLaws({
      cwd: this.root, groups: [
        { package: "semio-framework-async", target: { kind: "lib", name: "semio_framework_async" }, laws: ["native_drr_finishes_eligible_deficit_frontier_before_idle", "cooperative_maintenance_retains_deficit_until_later_host_turn"] },
        { package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, cargoArgs: ["--features", "sqlite"], laws: ["db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return", "db_io_saturated_task_retry_wakes_parked_caller_without_unrelated_ingress"] },
        { package: "semio-hub", target: { kind: "lib", name: "semio_hub" }, cargoArgs: ["--features", "sqlite"], laws: suffixes },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: buildBudgetMs(), listBudgetMs: 60_000, lawBudgetMs: 60_000,
      progress(event) { console.log(`gis-inference-ledger ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`gis-inference-ledger-receipt: ${JSON.stringify(receipt)}`);
    runCmd("cargo", ["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: 3_600_000 });
    console.log(`gis-inference-ledger-check: scheduler=2 memory-backing=1 parked-retry=1 sqlite=4 wal=5 catalog-projection=1 canonical-command=1 trusted-catalog=2 approval-request=1 live-author=1 gis-provider-selection=1 exact=${suffixes.length + 4}; no route/GIS-approval acceptance`);
  }
}

class GisMapFrozenBindingCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("gis-map-frozen-binding-check accepts only --native");
    await proveGisMapFrozenBindingFixture(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-hub", target: { kind: "lib", name: "semio_hub" }, laws: ["gis_map_verified_binding_freezes_catalog_selection_and_native_executable", "gis_map_binding_constructs_from_loaded_catalog_and_retains_verified_bytes"] }],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        listBudgetMs: 60_000,
        lawBudgetMs: 60_000,
        progress(event) { console.log(`gis-map-frozen-binding ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`gis-map-frozen-binding-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🗺️ The exact GIS Map proposal/approval gate: neutral oracle, native hub laws, honest nonclaims. */
class GisMapProposalCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const mode = segments[0] ?? "--source";
    if (segments.length > 1 || !["--source", "--native", "--process"].includes(mode)) throw new Error("usage: gis-map-proposal-check [--source|--native|--process]");
    const hostile = await proveGisMapProposalApprovalFixture(this.repoRoot);
    if (mode === "--native" || mode === "--process") {
      const libraryLaws = [
        "gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation",
        "gis_map_proposal_is_private_to_its_original_author_owner",
        "gis_map_approval_fails_closed_without_a_composition_transaction_and_never_auto_applies",
        "gis_map_proposal_fixture_pins_the_exact_frozen_comparison_limits_and_error_vocabulary",
      ];
      const routeLaws = ["gis_map_proposal_routes_fail_closed_without_a_trusted_map_binding"];
      const laws = [...libraryLaws, ...routeLaws];
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [
          { package: "semio-hub", target: { kind: "lib", name: "semio_hub" }, cargoArgs: ["--features", "sqlite"], laws: libraryLaws },
          { package: "semio-hub", target: { kind: "bin", name: "os-hub" }, laws: routeLaws },
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) { console.log(`gis-map-proposal ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`gis-map-proposal-receipt: ${JSON.stringify(receipt)}`);
      console.log(`gis-map-proposal-check: neutral hostile=${hostile} exact-native=${laws.length}; no external model provider, no WGPU rendering`);
    }
    if (mode === "--process") {
      console.log("gis-map-proposal-check --process: the two-user authenticated journey needs the trusted profile and the atomic composition transaction; it is NOT run or claimed here. No external model provider, no WGPU rendering.");
    }
    if (mode === "--source") console.log(`gis-map-proposal-check: neutral source oracle passed with hostile=${hostile}; native laws and the two-user process journey remain unclaimed. No external model provider, no WGPU rendering.`);
  }
}

class NativeDocumentOpenCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveDocumentOpenPlanFixture(this.repoRoot);
    proveNativeCredentialSourceOrder(this.repoRoot);
    runCmd("bun", ["nx", "run", "@semio-tech/framework-renderer-wgpu:check-frame-worker", "--skip-nx-cache"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run", "@semio-tech/framework-renderer-wgpu:native-build", "--skip-nx-cache", "--", "--scale"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runNativeDocumentAdmissionLaws(this.repoRoot);
    await proveNativeSocketGrantActor(this.repoRoot);
    console.log("native-document-open-check: independent D1 oracle, exact scope/package/surface laws, one-use plan receipt exchange, tag7 Session gate, actor stamping, reconnect reissue, cross-space isolation, cancellation, origin denial, and redaction passed");
  }
}

function runNativeDocumentAdmissionLaws(repoRoot: string): void {
  const target = ["test", "-p", "semio-framework-os-kernel", "--lib", "--features", "sync,ureq"];
  const suffixes = [
    "native_document_admission_issues_validates_and_exchanges_exactly_once",
    "hostile_or_cancelled_plan_never_reaches_receipt_exchange",
    "cancellation_after_receipt_exchange_never_reaches_a_document_socket",
    "mismatched_local_plugin_selection_never_exchanges_a_plan_receipt",
    "hostile_hub_binding_cannot_receive_a_credential_bound_document_grant",
    "native_terminal_connection_failure_clears_receipt_actor_before_reissue",
    "hub_document_actor_and_surface_authority_are_isolated_by_full_scope",
  ];
  const env = { ...process.env, RUST_MIN_STACK: "268435456" };
  for (const suffix of suffixes) {
    const listed = runProbe("cargo", [...target, suffix, "--", "--list"], { cwd: repoRoot, env, ...orchestratorBudgetOpts() });
    const matches = listed.stdout
      .split("\n")
      .filter((line) => line.endsWith(": test"))
      .map((line) => line.slice(0, -": test".length))
      .filter((name) => name.endsWith(suffix));
    if (listed.status !== 0 || matches.length !== 1)
      throw new Error(`native document admission gate expected exactly one ${suffix}, selected ${matches.length}; status=${listed.status}; diagnostic=${listed.stderr.trim().slice(-4_000) || "<none>"}`);
    runCargo([...target, matches[0]!, "--", "--exact", "--test-threads=1"], repoRoot, env);
  }
  const mcpSuffix = "mcp_probe_document_transport_binds_full_scope_and_exact_surface_authority";
  const mcpTarget = ["test", "-p", "semio-framework-os-mcp", "--lib"];
  const listed = runProbe("cargo", [...mcpTarget, mcpSuffix, "--", "--list"], { cwd: repoRoot, env, ...orchestratorBudgetOpts() });
  const matches = listed.stdout
    .split("\n")
    .filter((line) => line.endsWith(": test"))
    .map((line) => line.slice(0, -": test".length))
    .filter((name) => name.endsWith(mcpSuffix));
  if (listed.status !== 0 || matches.length !== 1)
    throw new Error(`MCP document admission gate expected exactly one ${mcpSuffix}, selected ${matches.length}; status=${listed.status}; diagnostic=${listed.stderr.trim().slice(-4_000) || "<none>"}`);
  runCargo([...mcpTarget, matches[0]!, "--", "--exact", "--test-threads=1"], repoRoot, env);
  console.log(`native-document-admission-laws: nativeExact=${suffixes.length} mcpExact=1 passed`);
}

/** 🔗️ `runCargo`'s `env` arg replaces `process.env` wholesale (see `runCmdInternal`'s
 * `opts.env ?? process.env`), so this inherits the full process env and only defaults the port —
 * otherwise the launcher's `OS_HUB_PORT`/`OS_HUB_DATA` (and `PATH`) would be silently dropped. */
class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    buildAdminSpa(this.repoRoot);
    runCargo(["build", "--manifest-path", "Cargo.toml"], this.root);
    const secureSuite = segments[0] === "secure-suite";
    const secureNative = secureSuite || segments[0] === "secure-native";
    const secureMcp = secureSuite || segments[0] === "secure-mcp";
    const secureAdmin = secureSuite || segments[0] === "secure-admin";
    if (secureNative) {
      proveNativeCredentialSourceOrder(this.repoRoot);
      runCmd("bun", ["nx", "run", "@semio-tech/framework-renderer-wgpu:native-build", "--skip-nx-cache", "--", process.env.SEMIO_PLUGIN ?? "s"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    }
    if (secureMcp) {
      proveMcpCredentialSourceOrder(this.repoRoot);
      runCmd("bun", ["nx", "run", "@semio-tech/framework-os-mcp-rs:build", "--skip-nx-cache"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    }
    const profiles: readonly LocalProfile[] = [
      { profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: secureSuite ? ["native", "mcp", "react-relay"] : ["native", "mcp"] },
      ...(secureAdmin ? [{ profileId: "administrator", subject: "local-administrator-01", displayName: "Local Administrator", allowedClientClasses: ["admin-relay"] as const }] : []),
    ];
    const dataRoot = resolve(process.env.OS_HUB_DATA ?? join(this.repoRoot, ".🧬semio", "🌐hub"));
    let trustedCatalog = trustedBootstrapCurrent(dataRoot);
    if (!trustedCatalog) {
      const receipt = await materializeTrustedStdioGisBundle(this.repoRoot, dataRoot);
      await validateAndPublishTrustedStdioGisCandidate(this.repoRoot, this.root, dataRoot, receipt);
      trustedCatalog = trustedBootstrapCurrent(dataRoot);
      if (!trustedCatalog) throw new Error("trusted stdio+GIS candidate did not publish an exact current generation");
    }
    const run = await startLocalHub(this.repoRoot, this.root, profiles, {
      port: Number(process.env[OS_HUB_PORT_ENV] ?? OS_HUB_PORT),
      dataDir: dataRoot,
      adminSubjects: secureAdmin ? ["semio.local.bootstrap/v1:local-administrator-01"] : undefined,
      trustedCatalog,
    });
    let relay: LocalBrowserRelay | undefined;
    let adminRelay: LocalAdminRelay | undefined;
    let ui: ChildProcess | undefined;
    let native: ChildProcess | undefined;
    let mcp: ChildProcess | undefined;
    const stop = (): void => {
      void relay?.stop();
      void adminRelay?.stop();
      if (ui?.exitCode === null) ui.kill();
      if (native?.exitCode === null) native.kill();
      if (mcp?.exitCode === null) mcp.kill();
      void finishLocalHub(run);
    };
    process.once("SIGINT", stop);
    process.once("SIGTERM", stop);
    try {
      await waitForReadiness(run);
      console.log(`[INFO] secure local hub ready at http://127.0.0.1:${run.port}`);
      if (secureNative) {
        const envelope = await issueLocalCredential(run, "developer", "native");
        native = await deliverNativeCredentialEnvelope(nativeWgpuExecutable(this.repoRoot), ["--plugin", process.env.SEMIO_PLUGIN ?? "s"], envelope, `http://127.0.0.1:${run.port}`);
        native.stdout?.pipe(process.stdout);
        native.stderr?.pipe(process.stderr);
        console.log("[INFO] secure native WGPU child started with an fd3 credential endpoint");
      }
      if (secureMcp) {
        const envelope = await issueLocalCredential(run, "developer", "mcp", secureNative ? 3 : 2);
        ({ child: mcp } = await startMcpWorkspaceChild(this.repoRoot, run, envelope));
        mcp.stdout?.pipe(process.stdout);
        mcp.stderr?.pipe(process.stderr);
        console.log("[INFO] secure MCP child started with an fd3 credential endpoint");
      }
      if (secureAdmin) {
        const envelope = await issueLocalCredential(run, "administrator", "admin-relay", secureSuite ? 5 : 2);
        const adminProof = randomBytes(32);
        const adminProofHex = adminProof.toString("hex");
        adminRelay = startLocalAdminRelay(`http://127.0.0.1:${run.port}`, envelope, adminProof);
        openExternalBrowser(`${adminRelay.url}/admin/#semio-admin=${adminProofHex}`);
        console.log(`[INFO] secure local admin starting at ${adminRelay.url}/admin/`);
      }
      if (secureSuite) {
        const uiPort = Number(process.env.S_OS_PORT ?? 6066);
        const uiOrigin = `http://127.0.0.1:${uiPort}`;
        const envelope = await issueLocalCredential(run, "developer", "react-relay", 4);
        const browserProof = randomBytes(32);
        const browserProofHex = browserProof.toString("hex");
        relay = startLocalBrowserRelay(`http://127.0.0.1:${run.port}`, uiOrigin, envelope, browserProof);
        const uiScript = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
        ui = spawn(process.execPath, [uiScript, "dev", "s"], {
          cwd: join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"),
          env: { ...process.env, S_OS_PORT: String(uiPort), S_HUB_URL: `http://127.0.0.1:${run.port}`, S_LOCAL_RELAY_URL: relay.url, S_LOCAL_RELAY_SECRET: relay.secret.toString("hex"), SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react" },
          shell: false,
          stdio: "inherit",
        });
        await waitForUiReadiness(uiOrigin, ui);
        openExternalBrowser(`${uiOrigin}/#semio-broker=${browserProofHex}`);
        console.log(`[INFO] secure local OS profile starting at ${uiOrigin}`);
      }
      await new Promise<void>((resolveExit, rejectExit) => {
        run.child.once("exit", (code) => (code === 0 ? resolveExit() : rejectExit(new Error(`hub child exited with status ${code}`))));
        ui?.once("exit", (code) => (code === 0 ? resolveExit() : rejectExit(new Error(`secure local UI child exited with status ${code}`))));
        native?.once("exit", (code) => (code === 0 ? resolveExit() : rejectExit(new Error(`secure native WGPU child exited with status ${code}`))));
        mcp?.once("exit", (code) => (code === 0 ? resolveExit() : rejectExit(new Error(`secure MCP child exited with status ${code}`))));
      });
    } finally {
      process.off("SIGINT", stop);
      process.off("SIGTERM", stop);
      await relay?.stop();
      await adminRelay?.stop();
      if (ui?.exitCode === null) ui.kill();
      if (native?.exitCode === null) native.kill();
      if (mcp?.exitCode === null) mcp.kill();
      await finishLocalHub(run);
    }
  }
}

class SecureLocalSmokeScript extends BundleScript {
  async run(): Promise<void> {
    await proveDocumentOpenPlanFixture(this.repoRoot);
    proveNativeCredentialSourceOrder(this.repoRoot);
    proveMcpCredentialSourceOrder(this.repoRoot);
    runCmd("bun", ["nx", "run", "@semio-tech/framework-renderer-wgpu:check-frame-worker", "--skip-nx-cache"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run", "@semio-tech/framework-renderer-wgpu:native-build", "--skip-nx-cache", "--", "--scale"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCmd("bun", ["nx", "run", "@semio-tech/framework-os-mcp-rs:build", "--skip-nx-cache"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runNativeDocumentAdmissionLaws(this.repoRoot);
    buildAdminSpa(this.repoRoot);
    runCargo(["build", "--manifest-path", "Cargo.toml"], this.root);
    await runSecureLocalSmoke(this.repoRoot, this.root);
    await proveNativeSocketGrantActor(this.repoRoot);
    console.log("native-mcp-socket-grant-check: neutral D1 oracle, exact native laws, direct children, early fd3 seals, byte-clean MCP stdio, strict open-plan receipt exchange, v1 grant/tag7, exact package/surface authority, actor stamping, forced reconnect, and fresh plan/grant reissue passed");
  }
}

class AdminRelayCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveAdminRelayBoundary(this.repoRoot);
    runCmd("bun", ["nx", "run", "os-hub-admin:test", "--skip-nx-cache", "--", "long", "--run", "🛡️admin.test.tsx"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    console.log("admin-relay-check: one-use fragment bootstrap, host-only HttpOnly strict cookie, expiry/replay/CSRF/raw-local denial, bearer redaction, and EN/DE UI laws passed");
  }
}

class TrustedStdioGisBundleCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments[0] !== undefined && segments[0] !== "--source" && segments[0] !== "--native" && segments[0] !== "--process")) throw new Error("usage: trusted-stdio-gis-bundle-check [--source|--native|--process]");
    await proveTrustedStdioGisBootstrapFixture(this.repoRoot);
    const describeSource = readFileSync(resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts"), "utf8");
    const producerStart = describeSource.indexOf("function freshCopy(");
    const producerEnd = describeSource.indexOf("\n/** @emoji 🛂️ Shared implementation", producerStart);
    const producer = describeSource.slice(producerStart, producerEnd);
    if (producerStart < 0 || producerEnd < 0 || !producer.includes("CARGO_INCREMENTAL: \"0\"") || !producer.includes("RUSTC_WRAPPER: \"\"") || !producer.includes("pluginWasmArtifactPath(") || !producer.includes("verifyFreshCatalogPackageV1(") || !producer.includes("freshCopy(") || !producer.includes("blake3Hex") || !producer.includes("if (!complete) rmSync(destination") || producer.indexOf("closeSync(output)") > producer.indexOf("if (!complete) rmSync(destination") || producer.includes("atomicDescriptorPair") || producer.includes("plugin-registry:generate") || producer.includes("ownerRoot")) throw new Error("fresh component producer is not isolated, descriptor-verified, bounded, close-before-cleanup, or side-effect free");
    const catalogSource = readFileSync(resolve(this.repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"), "utf8");
    const providerSource = readFileSync(resolve(this.repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs"), "utf8");
    const runtimeSource = readFileSync(resolve(this.repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8").split("\nmod tests {")[0]!;
    const registrySource = readFileSync(resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts"), "utf8");
    const runtimeCompact = runtimeSource.replace(/\s+/g, "");
    const receiptResolution = runtimeCompact.indexOf(".authority_for_authenticated_exchange(&intent.plan_receipt");
    const targetResolution = runtimeCompact.indexOf("catalog.resolve_document_open(&authority.descriptor", receiptResolution);
    if (!catalogSource.includes("bundle.schema_version != 2") || !catalogSource.includes("trusted_profile_generation(&bundle, &profile)") || !catalogSource.includes("selected profile must resolve exactly one document-open target") || !providerSource.includes("NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 28") || !providerSource.includes("receipt.package_version != version") || !registrySource.includes("CATALOG_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024") || runtimeSource.split("catalog.generation_id() != authority.catalog.generation_id").length - 1 !== 2 || receiptResolution < 0 || targetResolution < receiptResolution) throw new Error("trusted stdio+GIS runtime/source boundary is incomplete");
    const scriptSource = readFileSync(import.meta.path, "utf8");
    const body = (start: string, end: string): string => {
      const first = scriptSource.indexOf(start), last = scriptSource.indexOf(end, first);
      if (first < 0 || last < 0) throw new Error(`trusted stdio+GIS source boundary is missing ${start}`);
      return scriptSource.slice(first, last);
    };
    const materializer = body("\nasync function materializeTrustedStdioGisBundle", "\nfunction trustedBootstrapCurrent");
    const writer = body("\nfunction trustedBootstrapWriteNew", "\nfunction trustedBootstrapFsyncDirectory");
    const publisher = body("\nfunction publishTrustedBootstrapCurrent", "\n/** 🟢️ Publishes current metadata");
    const stalePlan = body("\nasync function proveTrustedStdioGisStalePlanRejected", "\n/** 🟢️ Publishes current metadata");
    const candidate = body("\nasync function validateAndPublishTrustedStdioGisCandidate", "\n/** ✉️ Independent bounded canonical envelope oracle");
    const rotation = body("\nasync function materializeTrustedStdioGisRotation", "\nfunction trustedBootstrapCurrent");
    const bootstrap = body("\nclass TrustedStdioGisBootstrapScript", "\nclass AdminBackendCheckScript");
    const dev = body("\nclass DevScript", "\nclass TrustedStdioGisBundleCheckScript");
    const nativeGate = body("\nclass TrustedStdioGisBundleCheckScript", "\nclass TrustedStdioGisBootstrapScript");
    const processGate = nativeGate.slice(nativeGate.lastIndexOf("if (segments[0] === \"--process\") {"));
    const ordered = (source: string, earlier: string, later: string): boolean => source.indexOf(earlier) >= 0 && source.indexOf(earlier) < source.indexOf(later);
    const missingFence = [
      ["materializer publication", !materializer.includes("publishTrustedBootstrapCurrent")],
      ["independent versions", !materializer.includes("stdio.version !== gis.version")],
      ["development publication", !dev.includes("publishTrustedBootstrapCurrent")],
      ["writer cleanup", ordered(writer, "closeSync(output)", "if (!complete) rmSync(path")],
      ["publisher cleanup", ordered(publisher, "renameSync(temporary", "rmSync(temporary")],
      ["candidate readiness", ordered(candidate, "await waitForReadiness(candidate)", "const plan = await proveTrustedStdioGisCandidatePlan(candidate, receipt, envelope)")],
      ["candidate publication", ordered(candidate, "const plan = await proveTrustedStdioGisCandidatePlan(candidate, receipt, envelope)", "publishTrustedBootstrapCurrent(dataRoot, receipt)")],
      ["candidate stale plan", ordered(candidate, "if (stalePlan) await proveTrustedStdioGisStalePlanRejected", "const plan = await proveTrustedStdioGisCandidatePlan")],
      ["stale receipt cleanup", stalePlan.includes("finally { stalePlan.receipt = \"\"; }")],
      ["rotation descriptor", rotation.includes("verifyFreshCatalogPackageV1(") && rotation.includes("trustedBootstrapProfileEncoding(")],
      ["server-owned rotation", rotation.includes("value.manifest.label = `Stdio trusted rotation ${randomBytes(8).toString(\"hex\")}`")],
      ["rotation publication", ordered(rotation, "renameSync(stageRoot, generationRoot)", "trustedBootstrapFsyncDirectory(generationsRoot)")],
      ["bootstrap candidate", ordered(bootstrap, "materializeTrustedStdioGisBundle", "validateAndPublishTrustedStdioGisCandidate")],
      ["development candidate", ordered(dev, "await materializeTrustedStdioGisBundle", "await validateAndPublishTrustedStdioGisCandidate")],
      ["development launch", ordered(dev, "await validateAndPublishTrustedStdioGisCandidate", "const run = await startLocalHub")],
      ["native target", nativeGate.includes("CARGO_TARGET_DIR: hubTarget") && nativeGate.includes("join(hubTarget, \"debug\"") && candidate.includes("dataDir: join(dataRoot, \"candidate-data\")")],
      ["process mode", processGate.length > 0],
      ["failed candidate", ordered(processGate, "const retained = trustedBootstrapCurrent(dataRoot)", "profileId: \"missing-profile\"")],
      ["restart candidate", ordered(processGate, "profileId: \"missing-profile\"", "const rotated = await materializeTrustedStdioGisRotation") && ordered(processGate, "const rotated = await materializeTrustedStdioGisRotation", "const freshPlan = await validateAndPublishTrustedStdioGisCandidate")],
    ].find(([, present]) => !present)?.[0];
    if (missingFence) throw new Error(`trusted stdio+GIS source fence is incomplete: ${missingFence}`);
    if (segments[0] === "--native" || segments[0] === "--process") {
      const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
      const ticketsRoot = resolve(this.repoRoot, ".🧬semio", "🦑️repo", "🎫️tickets");
      const artifactPath = artifactRoot ? resolve(artifactRoot) : "";
      const ticketRelative = artifactPath ? relative(ticketsRoot, artifactPath) : "";
      if (!artifactRoot || !isAbsolute(artifactRoot) || ticketRelative === "" || ticketRelative.startsWith("..") || isAbsolute(ticketRelative)) throw new Error("trusted stdio+GIS native gate requires an absolute ticket-owned SEMIO_TEST_ARTIFACT_DIR");
      mkdirSync(artifactRoot, { recursive: true, mode: 0o700 });
      const hubTarget = join(artifactPath, "hub-target");
      mkdirSync(hubTarget, { recursive: true, mode: 0o700 });
      const hubEnv = { ...process.env, CARGO_TARGET_DIR: hubTarget, CARGO_INCREMENTAL: "0", RUSTC_WRAPPER: "", SCCACHE_DISABLE: "1" };
      runCargo(["--config", 'build.rustc-wrapper=""', "build", "--manifest-path", "Cargo.toml", "--bin", "os-hub"], this.root, hubEnv);
      const dataRoot = join(resolve(artifactRoot), "server-owned-data");
      const receipt = await materializeTrustedStdioGisBundle(this.repoRoot, dataRoot);
      const binary = join(hubTarget, "debug", process.platform === "win32" ? "os-hub.exe" : "os-hub");
      const initialPlan = await validateAndPublishTrustedStdioGisCandidate(this.repoRoot, this.root, dataRoot, receipt, binary);
      if (segments[0] === "--process") {
        const retained = trustedBootstrapCurrent(dataRoot);
        if (!retained) throw new Error("trusted stdio+GIS process gate has no retained current generation");
        let rejected = false;
        try { await validateAndPublishTrustedStdioGisCandidate(this.repoRoot, this.root, dataRoot, { ...receipt, profileId: "missing-profile" }, binary); } catch { rejected = true; }
        if (!rejected || JSON.stringify(trustedBootstrapCurrent(dataRoot)) !== JSON.stringify(retained)) throw new Error("failed trusted stdio+GIS candidate changed the retained current generation");
        const rotated = await materializeTrustedStdioGisRotation(this.repoRoot, dataRoot, retained);
        const freshPlan = await validateAndPublishTrustedStdioGisCandidate(this.repoRoot, this.root, dataRoot, rotated, binary, initialPlan);
        if (JSON.stringify(trustedBootstrapCurrent(dataRoot)) !== JSON.stringify(rotated) || initialPlan.catalog.generationId !== retained.generationId || freshPlan.catalog.generationId !== rotated.generationId || retained.generationId === rotated.generationId) throw new Error("trusted stdio+GIS process rotation did not retain exact distinct plan generations");
        console.log("trusted-stdio-gis-bundle-process-check: failed candidate preserved current; a real next-generation candidate denied the old authenticated plan before issuing the fresh exact GIS Map plan");
      }
      console.log(`trusted-stdio-gis-bundle-native-receipt: ${JSON.stringify(receipt)}`);
      console.log("trusted-stdio-gis-bundle-check: actual isolated stdio+GIS generation loaded by a candidate hub and published current after readiness; client execution remains unclaimed");
      return;
    }
    console.log("trusted-stdio-gis-bundle-check: source+neutral exact closure passed; native materialization, candidate hub, and current pointer remain unclaimed");
  }
}

class TrustedStdioGisBootstrapScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("trusted-stdio-gis-bootstrap accepts no client-selected paths or profiles");
    const dataRoot = process.env.OS_HUB_DATA ? resolve(process.env.OS_HUB_DATA) : resolve(this.repoRoot, ".🧬semio", "🌐hub");
    runCargo(["build", "--manifest-path", "Cargo.toml", "--bin", "os-hub"], this.root);
    const receipt = await materializeTrustedStdioGisBundle(this.repoRoot, dataRoot);
    await validateAndPublishTrustedStdioGisCandidate(this.repoRoot, this.root, dataRoot, receipt);
    console.log(`trusted-stdio-gis-bootstrap-receipt: ${JSON.stringify(receipt)}`);
    console.log("trusted-stdio-gis-bootstrap: immutable generation loaded by a candidate and published current after readiness; client execution remains separate");
  }
}

class AdminBackendCheckScript extends BundleScript {
  async run(): Promise<void> {
    runCmd("bun", [join(this.repoRoot, "🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🧪️oracle/🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    const exactLaw = (target: string[], suffix: string): string => {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--all-features", ...target, suffix, "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`admin-backend-check expected exactly one ${suffix} law, selected ${matches.length}`);
      return matches[0]!;
    };
    const laws = [
      { target: ["--lib"], suffix: "admin_operation_audit_concurrent_first_writer_is_idempotent_and_first_terminal_wins" },
      { target: ["--lib"], suffix: "admin_bounded_overview_space_and_document_projections_enforce_exact_page_boundary" },
      { target: ["--bin", "os-hub"], suffix: "admin_intent_wire_taxonomy_rejects_generic_and_unknown_commands" },
      { target: ["--bin", "os-hub"], suffix: "admin_document_cursor_is_principal_route_and_exact_page_bound" },
      { target: ["--bin", "os-hub"], suffix: "admin_response_pages_stop_before_exact_byte_max_and_reject_one_oversized_row" },
      { target: ["--bin", "os-hub"], suffix: "admin_rebuild_slots_are_atomic_and_abort_closes_once" },
    ].map((law) => ({ ...law, name: exactLaw(law.target, law.suffix) }));
    const postgresLaw = { target: ["--lib"], suffix: "admin_operation_audit_concurrent_absent_request_rereads_established_receipt" };
    const postgres = { ...postgresLaw, name: exactLaw(postgresLaw.target, postgresLaw.suffix) };
    console.log(`admin-backend-laws: ${[...laws, postgres].map((law) => law.name).join(",")}`);
    for (const law of laws) runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", ...law.target, law.name, "--", "--exact", "--test-threads=1"], this.root);
    await proveAdminRelayBoundary(this.repoRoot);
    runCmd("bun", ["nx", "run", "os-hub-admin:test", "--skip-nx-cache", "--", "long", "--run", "🛡️admin.test.tsx"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", ...postgres.target, postgres.name, "--", "--exact", "--test-threads=1"], this.root);
    console.log("admin-backend-check: closed intents, verified principal, durable audit, bounded snapshots, rebuild lifecycle, relay, and SPA laws passed");
  }
}

class AdminLiveJourneyCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = await adminLiveJourneyFixture(this.repoRoot);
    const laws = [
      "local_bootstrap::tests::local_bootstrap_idle_listener_survives_until_admission_and_admitted_frame_is_deadline_bounded",
      "directory::sqlite::tests::projection_rebuild_preserves_live_credential_invite_and_session_bindings",
    ];
    for (const law of laws) {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--lib", law, "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
      const matches = listed.stdout.split("\n").filter((line) => line === `${law}: test`);
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`admin-live-journey-check expected exactly one ${law} law, selected ${matches.length}`);
      runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", law, "--", "--exact", "--test-threads=1"], this.root);
    }
    buildAdminSpa(this.repoRoot);
    runCargo(["build", "--manifest-path", "Cargo.toml", "--bin", "os-hub"], this.root);
    await proveAdminLiveJourney(this.repoRoot, this.root, fixture);
    console.log("admin-live-journey-check: protected loopback bootstrap/relay and real SQLite browser journey passed; no production OIDC claim");
  }
}

type InviteRedemptionFixture = {
  readonly schema: "semio.hub.invite-redemption-transaction/v1";
  readonly vectors: readonly {
    readonly name: string;
    readonly initial: "fresh" | "accepted-same" | "accepted-other" | "revoked" | "expired" | "missing" | "missing-user" | "missing-space" | "corrupt-marker" | "corrupt-event";
    readonly concurrent: boolean;
    readonly calls: readonly {
      readonly kind: "redeem" | "revoke" | "append-forged" | "restart" | "rebuild";
      readonly user: "same" | "other" | "missing";
      readonly actor: "exact" | "different";
      readonly credential: "exact" | "wrong-selector" | "wrong-secret";
      readonly failure: "none" | "after-marker" | "after-event" | "after-projection";
    }[];
    readonly expected: {
      readonly outcomes: readonly string[];
      readonly returnedEventIds: readonly (string | null)[];
      readonly marker: { readonly acceptedAt: number | null; readonly acceptedEventId: string | null };
      readonly events: number;
      readonly memberships: number;
      readonly publications: number;
      readonly replayEvents: number;
      readonly revoked: boolean;
    };
  }[];
  readonly hostiles: readonly { readonly name: string; readonly mutation: "raw-capability" | "client-space" | "client-role" | "client-event-id" | "unknown-field" | "oversized-identifier" }[];
};

type PresenceLeaseOperation =
  | { readonly kind: "install" | "tick" | "close"; readonly scope: string; readonly actor: string; readonly liveId: string; readonly nowMs: number }
  | { readonly kind: "refresh"; readonly scope: string; readonly actor: string; readonly liveId: string; readonly nowMs: number; readonly peerTag: string; readonly peerBytes: number }
  | { readonly kind: "fill"; readonly scope: string; readonly count: number; readonly peerBytes: number; readonly nowMs: number }
  | { readonly kind: "restart" };

type PresenceLeaseFixture = {
  readonly schema: "semio.hub.presence-lease/v1";
  readonly limits: { readonly ttlMs: 15000; readonly maximumItems: 64; readonly maximumEntryBytes: 4096; readonly maximumBytes: 262144 };
  readonly vectors: readonly {
    readonly name: string;
    readonly operations: readonly PresenceLeaseOperation[];
    readonly expected: {
      readonly outcomes: readonly string[];
      readonly fanoutCount: number;
      readonly final: readonly { readonly scope: string; readonly count: number; readonly bytes: number; readonly actors: readonly string[] }[];
      readonly socketStillLive: boolean;
      readonly durableWrites: 0;
      readonly directoryRecipients: readonly "member"[];
    };
  }[];
};

/** 👥️ Replays the server-local lease contract without using the Rust implementation. */
type DirectoryEventPageRouteFixture = {
  readonly schema: "semio.hub.directory-event-page-route/v1";
  readonly limits: { readonly rawRows: 128; readonly eventBytes: 49152; readonly pageBytes: 65536; readonly safeInteger: 9007199254740991 };
  readonly session: { readonly sessionId: string; readonly userId: string; readonly authorizationGeneration: number; readonly expiresAt: number; readonly bindingSha256: string };
  readonly vectors: readonly {
    readonly name: "raw-holes" | "128-hidden" | "page-byte-prefix" | "empty" | "event-byte-boundary";
    readonly after: number;
    readonly raw: readonly { readonly seq: number; readonly visible: boolean; readonly labelBytes: number }[];
    readonly expected: { readonly through: number; readonly hasMore: boolean; readonly visibleSeqs: readonly number[]; readonly nextVisibleSeqs: readonly number[] };
  }[];
  readonly queryCases: readonly { readonly query: string; readonly bearer: "valid" | "missing" | "revoked" | "rotated" | "backend-failure"; readonly status: number; readonly bodyBytes: number; readonly reads: number }[];
  readonly hostiles: readonly string[];
};

async function proveDirectoryEventPageRouteV1(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/🧪️fixtures/📇️directory/📅️event-page-route-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as DirectoryEventPageRouteFixture;
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`directory event page route fixture: ${JSON.stringify(validate.errors)}`);
  const u32be = (value: number): Buffer => { const bytes = Buffer.alloc(4); bytes.writeUInt32BE(value); return bytes; };
  const u64be = (value: number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
  const i64be = (value: number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigInt64BE(BigInt(value)); return bytes; };
  const sessionId = Buffer.from(fixture.session.sessionId, "utf8");
  const userId = Buffer.from(fixture.session.userId, "utf8");
  const binding = createHash("sha256")
    .update(Buffer.concat([
      Buffer.from("semio/hub/directory-event-page/session-binding/v1\0"),
      u32be(sessionId.length), sessionId,
      u32be(userId.length), userId,
      u64be(fixture.session.authorizationGeneration),
      i64be(fixture.session.expiresAt),
    ]))
    .digest("hex");
  if (binding !== fixture.session.bindingSha256) throw new Error("directory event page session binding is not byte-exact");
  const makeEvent = (row: { seq: number; visible: boolean; labelBytes: number }) => ({
    seq: row.seq,
    id: `event-${row.seq}`,
    hlc: { physicalMs: row.seq, logical: 0 },
    actor: { kind: "system", id: row.visible ? "system:visible" : "system:hidden-identity" },
    spaceId: row.visible ? "visible-space" : "hidden-space",
    body: { kind: "space.renamed", spaceId: row.visible ? "visible-space" : "hidden-space", name: "x".repeat(row.labelBytes) },
    recordedAtMs: row.seq,
  });
  const seal = (after: number, through: number, hasMore: boolean, events: readonly ReturnType<typeof makeEvent>[]) => {
    const unsigned = {
      schema: "semio.directory.event-page.v1",
      sessionBindingSha256: binding,
      authorizationGeneration: fixture.session.authorizationGeneration,
      afterSeqExclusive: after,
      throughSeqInclusive: through,
      hasMore,
      events,
    };
    const receiptSha256 = createHash("sha256").update(JSON.stringify(unsigned)).digest("hex");
    const page = { ...unsigned, receiptSha256 };
    return { page, bytes: Buffer.byteLength(JSON.stringify(page), "utf8") };
  };
  const build = (after: number, raw: readonly ReturnType<typeof makeEvent>[]) => {
    let through = after;
    const events: ReturnType<typeof makeEvent>[] = [];
    let stopped = false;
    for (const event of raw) {
      if (event.seq <= through || event.seq > fixture.limits.safeInteger || Buffer.byteLength(JSON.stringify(event), "utf8") > fixture.limits.eventBytes) throw new Error("directory event append rejected");
      if (event.spaceId === "hidden-space") {
        through = event.seq;
        continue;
      }
      const candidate = seal(after, event.seq, true, [...events, event]);
      if (candidate.bytes > fixture.limits.pageBytes) {
        stopped = true;
        break;
      }
      events.push(event);
      through = event.seq;
    }
    return seal(after, through, stopped || raw.length === fixture.limits.rawRows, events).page;
  };
  for (const vector of fixture.vectors) {
    if (vector.name === "event-byte-boundary") {
      const exact = makeEvent(vector.raw[0]!);
      const plusOne = makeEvent(vector.raw[1]!);
      if (Buffer.byteLength(JSON.stringify(exact), "utf8") !== fixture.limits.eventBytes) throw new Error("exact event boundary fixture drifted");
      if (Buffer.byteLength(JSON.stringify(plusOne), "utf8") !== fixture.limits.eventBytes + 1) throw new Error("event max+1 fixture drifted");
      if (build(vector.after, [exact]).events.length !== 1) throw new Error("exact event boundary was not admitted");
      try { build(vector.after, [plusOne]); throw new Error("event max+1 admitted"); } catch (error) { if ((error as Error).message !== "directory event append rejected") throw error; }
      continue;
    }
    const rows = vector.name === "128-hidden"
      ? Array.from({ length: fixture.limits.rawRows }, (_, index) => makeEvent({ seq: vector.after + index + 1, visible: false, labelBytes: 8 }))
      : vector.raw.map(makeEvent);
    const page = build(vector.after, rows);
    if (page.throughSeqInclusive !== vector.expected.through || page.hasMore !== vector.expected.hasMore || JSON.stringify(page.events.map(event => event.seq)) !== JSON.stringify(vector.expected.visibleSeqs)) {
      throw new Error(`directory event page vector differs for ${vector.name}`);
    }
    if (page.events.some(event => JSON.stringify(event).includes("hidden-identity"))) throw new Error("hidden raw identity leaked into a page");
    const receipt = createHash("sha256").update(JSON.stringify({ ...page, receiptSha256: undefined }, (_key, value) => value)).digest("hex");
    if (receipt !== page.receiptSha256 || Buffer.byteLength(JSON.stringify(page), "utf8") > fixture.limits.pageBytes) throw new Error(`directory event page receipt/size differs for ${vector.name}`);
    if (vector.expected.nextVisibleSeqs.length > 0) {
      const next = build(page.throughSeqInclusive, rows.filter(row => row.seq > page.throughSeqInclusive));
      if (JSON.stringify(next.events.map(event => event.seq)) !== JSON.stringify(vector.expected.nextVisibleSeqs)) throw new Error("directory event page continuation skipped a visible row");
    }
  }
  const parseAfter = (query: string): number | null => {
    if (query.includes("&") || query.includes("%") || query.includes("+")) return null;
    const parts = query.split("=");
    if (parts.length !== 2 || parts[0] !== "after" || !/^(0|[1-9][0-9]*)$/.test(parts[1]!)) return null;
    const value = Number(parts[1]);
    return Number.isSafeInteger(value) && value <= fixture.limits.safeInteger ? value : null;
  };
  for (const query of fixture.queryCases) {
    const admitted = parseAfter(query.query) !== null;
    const status = !admitted ? 400 : query.bearer === "valid" ? 200 : query.bearer === "backend-failure" ? 500 : 401;
    const reads = admitted && ["valid", "rotated", "backend-failure"].includes(query.bearer) ? 1 : 0;
    if (status !== query.status || reads !== query.reads || ((status === 200 ? 1 : 0) !== query.bodyBytes)) throw new Error(`directory event page query result differs for ${query.query}`);
  }
  const hostilePage = seal(0, 0, false, []).page;
  if (createHash("sha256").update(JSON.stringify({ ...hostilePage, receiptSha256: undefined })).digest("hex") === "b".repeat(64)) throw new Error("receipt substitution was not rejected");
  const alternateSession = Buffer.from("session-route-0", "utf8");
  const alternateUser = Buffer.from("1user-route-01", "utf8");
  if (Buffer.concat([sessionId, userId]).compare(Buffer.concat([alternateSession, alternateUser])) !== 0) throw new Error("binding ambiguity fixture drifted");
  const alternateBinding = createHash("sha256")
    .update(Buffer.concat([Buffer.from("semio/hub/directory-event-page/session-binding/v1\0"), u32be(alternateSession.length), alternateSession, u32be(alternateUser.length), alternateUser, u64be(fixture.session.authorizationGeneration), i64be(fixture.session.expiresAt)]))
    .digest("hex");
  if (alternateBinding === binding) throw new Error("length-prefixed session binding aliased concatenated identities");
  const unknownFixture = { ...fixture, unknown: true };
  if (validate(unknownFixture)) throw new Error("directory event page route schema admitted an unknown field");
  const shared = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"), "utf8");
  const hub = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");
  const sqlite = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🪶️sqlite/🦀️.rs"), "utf8");
  const postgres = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🐘️postgres/🦀️.rs"), "utf8");
  const neo4j = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🌐️neo4j/🦀️.rs"), "utf8");
  const sourceClosed = (contract: string, route: string, sq: string, pg: string, neo: string): boolean => {
    const read = route.indexOf(".events_since(after, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS)");
    const revalidate = route.indexOf("revalidate_directory_event_page_caller(state, caller, binding)", read);
    const sqliteValidate = sq.indexOf("validate_directory_event_page_event(&persisted)", sq.indexOf("fn persist_event_with_identity"));
    const postgresValidates = [...pg.matchAll(/validate_directory_event_page_event\(&(?:persisted|full)\)/g)].map(match => match.index ?? -1);
    const neoValidates = [...neo.matchAll(/validate_directory_event_page_event\(&(?:persisted|full)\)/g)].map(match => match.index ?? -1);
    return contract.includes("pub fn validate_directory_event_page_event")
      && contract.includes("validate_directory_event_page_event(event).is_err()")
      && route.includes("fn directory_event_page_request_admission")
      && route.includes("semio/hub/directory-event-page/session-binding/v1\\0")
      && route.includes("query.contains('&') || query.contains('%') || query.contains('+')")
      && route.includes("value.len() > 1 && value.starts_with('0')")
      && read >= 0 && revalidate > read
      && route.includes("stopped_for_bytes || raw_len == DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS")
      && route.includes("struct DirectoryEventPageHttpRequest")
      && route.includes("if !self.response_owned")
      && route.includes("control.checkpoint()?")
      && route.includes("let visible = directory_event_page_event_visible(state, &event, &caller).await?")
      && route.includes("get_role(space_id, &caller.user_id).await.map_err(directory_error_status)?")
      && route.includes("tokio::time::timeout(std::time::Duration::from_millis(DIRECTORY_EVENT_PAGE_DEADLINE_MS)")
      && route.includes(".route(\"/directory/event-page/v1\", get(get_directory_event_page_v1))")
      && sqliteValidate >= 0 && sqliteValidate > sq.indexOf("INSERT INTO hub_directory_event", sq.indexOf("fn persist_event_with_identity"))
      && postgresValidates.length === 3 && neoValidates.length === 3
      && postgresValidates.every(index => pg.indexOf("INSERT INTO hub_directory_event", index) > index)
      && neoValidates.every(index => neo.indexOf("CREATE (e:DirectoryEvent", index) > index);
  };
  if (!sourceClosed(shared, hub, sqlite, postgres, neo4j)) throw new Error("directory event page route/storage source boundary is incomplete");
  const sourceHostiles: readonly [string, string, string, string, string][] = [
    [shared.replace("pub fn validate_directory_event_page_event", "fn validate_directory_event_page_event"), hub, sqlite, postgres, neo4j],
    [shared, hub.replace(".events_since(after, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS)", ".events_since(after, DIRECTORY_EVENT_READ_MAX)"), sqlite, postgres, neo4j],
    [shared, hub.replace("revalidate_directory_event_page_caller(state, caller, binding)", "Ok(caller.clone())"), sqlite, postgres, neo4j],
    [shared, hub.replace("query.contains('&') || query.contains('%') || query.contains('+')", "false"), sqlite, postgres, neo4j],
    [shared, hub.replaceAll("control.checkpoint()?;", ""), sqlite, postgres, neo4j],
    [shared, hub, sqlite.replace("validate_directory_event_page_event(&persisted)", "Ok(())"), postgres, neo4j],
    [shared, hub, sqlite, postgres.replace("validate_directory_event_page_event(&full)", "Ok(())"), neo4j],
    [shared, hub, sqlite, postgres, neo4j.replace("validate_directory_event_page_event(&full)", "Ok(())")],
    [shared, hub.replace("let visible = directory_event_page_event_visible(state, &event, &caller).await?", "let visible = event_visible(state, &event, Some(&caller)).await"), sqlite, postgres, neo4j],
  ];
  sourceHostiles.forEach((candidate, index) => { if (sourceClosed(...candidate)) throw new Error(`directory event page source oracle admitted removed fence ${index}`); });
  const runner = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/📜️script.ts"), "utf8");
  const processBody = (source: string): string => {
    const start = source.lastIndexOf("type LiveDirectoryEventPageV1 = {");
    const end = source.indexOf("\nclass DirectoryEventPageV1CheckScript", start);
    return start >= 0 && end > start ? source.slice(start, end) : "";
  };
  const withoutProcessFence = (source: string, needle: string, replacement: string): string => {
    const start = source.lastIndexOf("type LiveDirectoryEventPageV1 = {");
    const index = source.indexOf(needle, start);
    return index < 0 ? source : `${source.slice(0, index)}${replacement}${source.slice(index + needle.length)}`;
  };
  const processClosed = (source: string): boolean => {
    const body = processBody(source);
    const command = source.slice(source.indexOf("class DirectoryEventPageV1CheckScript"));
    return body.match(/dataDir: dataRoot/g)?.length === 2
      && body.includes("second = await startLocalHub")
      && body.includes("page.receiptSha256 !== receipt")
      && body.includes("stale.status !== 401")
      && body.includes("saturated.events.length !== 0")
      && command.includes("await proveDirectoryEventPageV1Process(this.repoRoot, this.root)");
  };
  const processHostiles = [
    withoutProcessFence(runner, "dataDir: dataRoot", "dataDir: undefined"),
    withoutProcessFence(runner, "page.receiptSha256 !== receipt", "false"),
    withoutProcessFence(runner, "stale.status !== 401", "false"),
    withoutProcessFence(runner, "second = await startLocalHub", "second = await Promise.reject"),
  ];
  if (!processClosed(runner)) throw new Error("directory event page real-process boundary is incomplete");
  processHostiles.forEach((candidate, index) => { if (processClosed(candidate)) throw new Error(`directory event page process oracle admitted removed fence ${index}`); });
  const checks = fixture.vectors.length + fixture.queryCases.length + fixture.hostiles.length + sourceHostiles.length + processHostiles.length + 2;
  console.log(`directory-event-page-v1-oracle: AJV=1 vectors=${fixture.vectors.length} queries=${fixture.queryCases.length} hostiles=${fixture.hostiles.length} source-hostiles=${sourceHostiles.length} process-hostiles=${processHostiles.length} sha256=1`);
  return checks;
}

type LiveDirectoryEventPageV1 = {
  readonly schema: "semio.directory.event-page.v1";
  readonly sessionBindingSha256: string;
  readonly authorizationGeneration: number;
  readonly afterSeqExclusive: number;
  readonly throughSeqInclusive: number;
  readonly hasMore: boolean;
  readonly events: readonly Record<string, any>[];
  readonly receiptSha256: string;
};

function liveDirectoryEventPageBinding(envelope: Record<string, any>, user: Record<string, any>): string {
  const session = Buffer.from(String(envelope.sessionId), "utf8");
  const userId = Buffer.from(String(user.userId), "utf8");
  const u32 = (value: number): Buffer => { const bytes = Buffer.alloc(4); bytes.writeUInt32BE(value); return bytes; };
  const u64 = (value: number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
  const i64 = (value: number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigInt64BE(BigInt(value)); return bytes; };
  return createHash("sha256").update(Buffer.concat([
    Buffer.from("semio/hub/directory-event-page/session-binding/v1\0"),
    u32(session.length), session,
    u32(userId.length), userId,
    u64(user.authorizationGeneration),
    i64(user.expiresAt),
  ])).digest("hex");
}

async function liveDirectoryEventPageUser(run: LocalHubRun, envelope: Record<string, any>): Promise<Record<string, any>> {
  const response = await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, {
    headers: { authorization: `Bearer ${envelope.capability}` },
    signal: AbortSignal.timeout(2_000),
  });
  const user = await response.json().catch(() => undefined) as Record<string, any> | undefined;
  if (!response.ok || !user || typeof user.userId !== "string" || user.userId.length === 0 || typeof user.displayName !== "string" || user.displayName.length === 0 || user.expiresAt !== envelope.expiresAt || user.authorizationGeneration !== envelope.authorizationGeneration) {
    throw new Error("directory event page process session identity did not match its inherited envelope");
  }
  return user;
}

async function fetchLiveDirectoryEventPage(run: LocalHubRun, envelope: Record<string, any>, user: Record<string, any>, after: number): Promise<LiveDirectoryEventPageV1> {
  const response = await fetch(`http://127.0.0.1:${run.port}/directory/event-page/v1?after=${after}`, {
    headers: { authorization: `Bearer ${envelope.capability}` },
    signal: AbortSignal.timeout(2_000),
  });
  const source = await response.text();
  if (!response.ok || response.headers.get("content-type")?.split(";", 1)[0] !== "application/json" || Buffer.byteLength(source, "utf8") > 64 * 1024) {
    throw new Error(`directory event page process read failed: ${response.status}:${Buffer.byteLength(source, "utf8")}`);
  }
  const page = JSON.parse(source) as LiveDirectoryEventPageV1;
  if (JSON.stringify(page) !== source || JSON.stringify(Object.keys(page)) !== JSON.stringify(["schema", "sessionBindingSha256", "authorizationGeneration", "afterSeqExclusive", "throughSeqInclusive", "hasMore", "events", "receiptSha256"])) {
    throw new Error("directory event page process response was not exact canonical JSON");
  }
  const unsigned = {
    schema: page.schema,
    sessionBindingSha256: page.sessionBindingSha256,
    authorizationGeneration: page.authorizationGeneration,
    afterSeqExclusive: page.afterSeqExclusive,
    throughSeqInclusive: page.throughSeqInclusive,
    hasMore: page.hasMore,
    events: page.events,
  };
  const receipt = createHash("sha256").update(JSON.stringify(unsigned)).digest("hex");
  if (page.schema !== "semio.directory.event-page.v1" || page.afterSeqExclusive !== after || page.authorizationGeneration !== user.authorizationGeneration || page.sessionBindingSha256 !== liveDirectoryEventPageBinding(envelope, user) || page.receiptSha256 !== receipt) {
    throw new Error("directory event page process receipt/session binding did not verify independently");
  }
  return page;
}

/** 🆔️ Mints one fresh 32-hex nonzero idempotency correlation for a live process command. */
function liveDirectoryCommandRequestId(): string {
  return randomBytes(16).toString("hex");
}

/** 🧾️ Posts one sealed `DirectoryCommandRequestV1` and returns the raw status plus response text —
 * the caller decides whether a non-2xx or a redacted receipt is the expected law. */
async function postLiveDirectoryCommand(run: LocalHubRun, capability: string, requestId: string, command: Record<string, unknown>, body?: string): Promise<{ readonly status: number; readonly text: string }> {
  const response = await fetch(`http://127.0.0.1:${run.port}/directory/commands`, {
    method: "POST",
    headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" },
    body: body ?? JSON.stringify({ schema: "semio.directory.command-request.v1", requestId, command }),
    signal: AbortSignal.timeout(5_000),
  });
  return { status: response.status, text: await response.text() };
}

async function submitLiveDirectoryCommand(run: LocalHubRun, envelope: Record<string, any>, command: Record<string, unknown>): Promise<readonly Record<string, any>[]> {
  const requestId = liveDirectoryCommandRequestId();
  const { status, text } = await postLiveDirectoryCommand(run, envelope.capability, requestId, command);
  const receipt = status === 202 ? JSON.parse(text) as Record<string, any> : undefined;
  if (!receipt || receipt.schema !== "semio.directory.command-receipt.v1" || receipt.requestId !== requestId || receipt.outcome !== "accepted" || !Array.isArray(receipt.events) || receipt.events.length === 0) {
    throw new Error(`directory process command failed: ${status}`);
  }
  return receipt.events;
}

function createdLiveDirectorySpace(events: readonly Record<string, any>[]): string {
  const id = events.find((event) => event?.body?.kind === "space.created")?.body?.spaceId;
  if (typeof id !== "string" || id.length === 0) throw new Error("directory event page process create-space result lacked its exact id");
  return id;
}

type DirectoryHomeBrowserProcessFixture = {
  readonly schema: "semio.hub.directory-home-browser-process-fixture/v1";
  readonly limits: { readonly journeyMs: number; readonly stepMs: number; readonly responseBytes: 65536; readonly appliedEvents: number };
  readonly spaceGuest: { readonly target: "wasm32-wasip2"; readonly package: "semio-s-plugin-space"; readonly nativeFeature: "os-host-full"; readonly forbiddenPackages: readonly ["ring", "cc", "tokio"] };
  readonly profiles: Readonly<Record<"a" | "b", { readonly profileId: string; readonly subject: string; readonly displayName: string }>>;
  readonly home: { readonly pluginId: "s"; readonly appId: "s.space.home@1/*#editor"; readonly actionId: "applyDirectoryEventPage"; readonly moduleDirectory: "🪐️s" };
  readonly pages: Readonly<Record<string, { readonly epoch: number; readonly binding: string; readonly generation: number; readonly after: number; readonly through: number; readonly hasMore: boolean; readonly receipt: string; readonly eventIds: readonly string[] }>>;
  readonly traces: readonly { readonly name: string; readonly steps: readonly Record<string, any>[]; readonly expected: Record<string, any> }[];
  readonly hostile: readonly { readonly name: string; readonly mutation: string }[];
};

function directoryHomeBrowserProcessModel(fixture: DirectoryHomeBrowserProcessFixture): number {
  type State = { epoch: number; frontier: number; phase: "fetching" | "awaiting-ack" | "live" | "closed"; pending?: DirectoryHomeBrowserProcessFixture["pages"][string]; applied: string[]; dials: number[]; retries: number[]; denials: number };
  const exactPage = (left: DirectoryHomeBrowserProcessFixture["pages"][string] | undefined, right: DirectoryHomeBrowserProcessFixture["pages"][string]): boolean => !!left
    && left.epoch === right.epoch && left.binding === right.binding && left.generation === right.generation && left.after === right.after && left.through === right.through && left.receipt === right.receipt;
  const step = (state: State, operation: Record<string, any>): void => {
    if (operation.kind === "rebootstrap") {
      state.epoch = operation.epoch;
      state.frontier = 0;
      state.phase = "fetching";
      state.pending = undefined;
      state.applied = [];
      return;
    }
    if (operation.kind === "close") {
      state.phase = "closed";
      state.pending = undefined;
      return;
    }
    if (operation.kind === "wake") {
      if (state.phase === "live") state.phase = "fetching";
      else state.denials += 1;
      return;
    }
    if (operation.kind === "dial") {
      if (state.phase === "live" && operation.since === state.frontier) state.dials.push(operation.since);
      else state.denials += 1;
      return;
    }
    const page = fixture.pages[operation.page];
    if (!page) throw new Error(`directory Home process trace references unknown page ${operation.page}`);
    if (operation.kind === "present") {
      if (state.phase === "fetching" && page.epoch === state.epoch && page.after === state.frontier && page.through >= state.frontier) {
        state.pending = page;
        state.phase = "awaiting-ack";
      } else state.denials += 1;
      return;
    }
    if (operation.kind === "reject") {
      if (state.phase === "awaiting-ack" && exactPage(state.pending, page)) {
        state.pending = undefined;
        state.phase = "fetching";
        state.retries.push(state.frontier);
      } else state.denials += 1;
      return;
    }
    if (operation.kind !== "ack" || state.phase !== "awaiting-ack" || !exactPage(state.pending, page)) {
      state.denials += 1;
      return;
    }
    if (page.eventIds.some((id) => state.applied.includes(id)) || state.applied.length + page.eventIds.length > fixture.limits.appliedEvents) {
      state.denials += 1;
      return;
    }
    state.applied.push(...page.eventIds);
    state.frontier = page.through;
    state.pending = undefined;
    state.phase = page.hasMore ? "fetching" : "live";
  };
  for (const trace of fixture.traces) {
    const state: State = { epoch: 1, frontier: 0, phase: "fetching", applied: [], dials: [], retries: [], denials: 0 };
    for (const operation of trace.steps) step(state, operation);
    const observed = { epoch: state.epoch, frontier: state.frontier, phase: state.phase, appliedEventIds: state.applied, dials: state.dials, retries: state.retries, denials: state.denials };
    if (JSON.stringify(observed) !== JSON.stringify(trace.expected)) throw new Error(`directory Home process model differs for ${trace.name}: ${JSON.stringify(observed)}`);
  }
  const initial = fixture.pages.initial!;
  const mutations: Record<string, (page: typeof initial) => typeof initial> = {
    epoch: (page) => ({ ...page, epoch: page.epoch + 1 }),
    binding: (page) => ({ ...page, binding: "f".repeat(64) }),
    generation: (page) => ({ ...page, generation: page.generation + 1 }),
    through: (page) => ({ ...page, through: page.through + 1 }),
    receipt: (page) => ({ ...page, receipt: "f".repeat(64) }),
  };
  for (const hostile of fixture.hostile) {
    const state: State = { epoch: 1, frontier: 0, phase: "fetching", applied: [], dials: [], retries: [], denials: 0 };
    if (mutations[hostile.mutation]) {
      step(state, { kind: "present", page: "initial" });
      const changed = mutations[hostile.mutation]!(initial);
      if (state.phase !== "awaiting-ack" || exactPage(state.pending, changed)) throw new Error(`directory Home process hostile ${hostile.name} was not distinct`);
      state.denials += 1;
    } else if (hostile.mutation === "duplicate-event") {
      state.pending = { ...initial, eventIds: [initial.eventIds[0]!, initial.eventIds[0]!] };
      state.phase = "awaiting-ack";
      if (new Set(state.pending.eventIds).size === state.pending.eventIds.length) throw new Error("directory Home duplicate hostile was not duplicated");
      state.denials += 1;
    } else if (hostile.mutation === "socket-before-ack") {
      step(state, { kind: "present", page: "initial" });
      step(state, { kind: "dial", since: 0 });
    } else if (hostile.mutation === "late-after-close") {
      step(state, { kind: "present", page: "initial" });
      step(state, { kind: "close" });
      step(state, { kind: "ack", page: "initial" });
    } else throw new Error(`directory Home process model has unknown hostile ${hostile.mutation}`);
    if (state.frontier !== 0 || state.applied.length !== 0 || state.denials !== 1) throw new Error(`directory Home process hostile ${hostile.name} advanced authority`);
  }
  return fixture.traces.length + fixture.hostile.length;
}

async function proveDirectoryHomeBrowserProcessSource(repoRoot: string): Promise<DirectoryHomeBrowserProcessFixture> {
  const fixtureRoot = join(repoRoot, "🌎️hub/📇️directory/🧫️fixtures/🌐️directory-home-browser-process-v1");
  const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as DirectoryHomeBrowserProcessFixture;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`directory Home browser process fixture invalid: ${JSON.stringify(validate.errors)}`);
  const modelChecks = directoryHomeBrowserProcessModel(fixture);
  const guestTree = runProbe("cargo", ["tree", "-e", "features", "-p", fixture.spaceGuest.package, "--target", fixture.spaceGuest.target], { cwd: repoRoot, budgetMs: fixture.limits.stepMs });
  if (guestTree.status !== 0) throw new Error(`directory Home Space guest graph oracle failed: ${guestTree.stderr}`);
  for (const forbidden of fixture.spaceGuest.forbiddenPackages) {
    if (new RegExp(`(?:^|\\s)${forbidden} v`, "mu").test(guestTree.stdout)) throw new Error(`directory Home Space guest graph retained forbidden package ${forbidden}`);
  }
  if (guestTree.stdout.includes(`semio-framework-os feature "${fixture.spaceGuest.nativeFeature}"`)) throw new Error(`directory Home Space guest graph retained native feature ${fixture.spaceGuest.nativeFeature}`);
  const rustc = runProbe("rustc", ["-vV"], { cwd: repoRoot, budgetMs: fixture.limits.stepMs });
  const hostTarget = rustc.status === 0 ? rustc.stdout.match(/^host: (.+)$/mu)?.[1] : undefined;
  if (!hostTarget) throw new Error(`directory Home Space native target oracle failed: ${rustc.stderr}`);
  const nativeTree = runProbe("cargo", ["tree", "-e", "features", "-p", fixture.spaceGuest.package, "--target", hostTarget, "-i", "semio-framework-os"], { cwd: repoRoot, budgetMs: fixture.limits.stepMs });
  if (nativeTree.status !== 0 || !nativeTree.stdout.includes(`semio-framework-os feature "${fixture.spaceGuest.nativeFeature}"`)) throw new Error("directory Home Space native graph lost os-host-full");
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  const owner = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx"), "utf8");
  const shell = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"), "utf8");
  const sourceClosed = (workerSource: string, ownerSource: string, shellSource: string): boolean => {
    const acknowledge = workerSource.indexOf("owner.machine.acknowledge(ack)");
    const openLive = workerSource.indexOf("openDirectoryBootstrapLive(owner, transition.since)");
    const identity = ownerSource.indexOf('directoryActionInvocation(owner, "setClient"');
    const opened = ownerSource.indexOf('kind: "directory-bootstrap-open"');
    const receipt = ownerSource.indexOf("parseDirectoryProjectionReceiptV1(response.output)");
    const ack = ownerSource.indexOf('kind: "directory-bootstrap-ack"');
    return workerSource.includes("class DirectoryEventPageBootstrapV1")
      && workerSource.includes("streamAcknowledged(since")
      && acknowledge >= 0 && openLive > acknowledge
      && workerSource.includes("if (directoryBootstrap !== owner || owner.abort.signal.aborted) return")
      && workerSource.includes("owner.machine.wake(rebootstrap)")
      && ownerSource.includes("await owner.plugin.handleAction")
      && identity >= 0 && opened > identity
      && ownerSource.includes("invocationTerminal(response)")
      && ownerSource.includes("if (owner.ownsInstance)")
      && ownerSource.includes("await beforeAcknowledge?.(owner)")
      && receipt >= 0 && ack > receipt
      && ownerSource.includes("owner.abort.signal.aborted")
      && ownerSource.includes('kind: "directory-bootstrap-reject"')
      && shellSource.includes("openDirectoryHomeOwnerV1")
      && shellSource.includes("instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState }")
      && shellSource.includes("identity: { userId: identity.userId, displayName: identity.displayName }")
      && shellSource.includes("directoryHomeOpeningRef.current.catch")
      && shellSource.includes("await refreshDirectoryHomeRef.current(active)")
      && shellSource.includes("applyDirectoryEventPageBootstrapV1")
      && shellSource.includes("closeDirectoryHomeOwnerV1")
      && !shellSource.includes('kind: "directory-open", baseUrl: resolved.hubBaseUrl');
  };
  if (!sourceClosed(worker, owner, shell)) throw new Error("directory Home browser process source lost ACK-owned frontier/cancellation wiring");
  const hostiles = [
    [worker.replace("owner.machine.acknowledge(ack)", "owner.machine.wake(false)"), owner, shell],
    [worker.replaceAll("if (directoryBootstrap !== owner || owner.abort.signal.aborted) return", "if (false) return"), owner, shell],
    [worker, owner.replace("parseDirectoryProjectionReceiptV1(response.output)", "page as any"), shell],
    [worker, owner.replaceAll("owner.abort.signal.aborted", "false"), shell],
    [worker, owner.replace('directoryActionInvocation(owner, "setClient"', 'directoryActionInvocation(owner, "applyDirectoryEventPage"'), shell],
    [worker, owner, shell.replace("instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState }", "instance: undefined")],
  ];
  hostiles.forEach((candidate, index) => { if (sourceClosed(candidate[0]!, candidate[1]!, candidate[2]!)) throw new Error(`directory Home browser process source oracle admitted removed fence ${index}`); });
  console.log(`directory-home-browser-process-oracle: ajv=1 model=${modelChecks} source=5 hostile-source=${hostiles.length} guest-graph=${fixture.spaceGuest.forbiddenPackages.length} native-feature=1 passed`);
  return fixture;
}

async function proveDirectoryHomeBrowserControllerRuntime(repoRoot: string, fixture: DirectoryHomeBrowserProcessFixture): Promise<void> {
  let runtimeServer: { stop(closeActiveConnections?: boolean): void | Promise<void> } | undefined;
  let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
  const browserDiagnostics: string[] = [];
  const abort = AbortSignal.timeout(fixture.limits.journeyMs);
  try {
    process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(repoRoot, "node_modules", ".cache", "ms-playwright");
    const controllerPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx");
    const bundle = await Bun.build({ entrypoints: [controllerPath], target: "browser", format: "esm", sourcemap: "none" });
    if (!bundle.success || bundle.outputs.length !== 1) throw new Error(`directory Home browser controller bundle failed: ${bundle.logs.map(String).join("; ") || "unexpected output closure"}`);
    const controller = await bundle.outputs[0]!.text();
    const port = await freeLoopbackPort();
    runtimeServer = Bun.serve({
      hostname: "127.0.0.1",
      port,
      fetch(request): Response {
        const path = new URL(request.url).pathname;
        if (path === "/controller.js") return new Response(controller, { headers: { "content-type": "text/javascript; charset=utf-8", "cache-control": "no-store" } });
        if (path === "/__directory-home-controller") return new Response("<!doctype html><meta charset=utf-8><title>Directory Home Controller</title>", { headers: { "content-type": "text/html; charset=utf-8" } });
        return new Response("not found", { status: 404 });
      },
    });
    const { chromium } = await import("playwright");
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    page.on("console", (message) => browserDiagnostics.push(`console:${message.type()}:${message.text()}`));
    page.on("pageerror", (error) => browserDiagnostics.push(`pageerror:${error.message}`));
    page.on("requestfailed", (request) => browserDiagnostics.push(`requestfailed:${request.url()}:${request.failure()?.errorText ?? "unknown"}`));
    const probeUrl = `http://127.0.0.1:${port}/__directory-home-controller`;
    await page.goto(probeUrl, { waitUntil: "domcontentloaded", timeout: fixture.limits.stepMs });
    const moduleUrl = "/controller.js";
    const deadline = new Promise<never>((_, reject) => {
      const fail = () => reject(new Error("directory Home browser controller deadline exceeded"));
      if (abort.aborted) fail(); else abort.addEventListener("abort", fail, { once: true });
    });
    const result = await Promise.race([
      page.evaluate(async ({ moduleUrl, home, sourcePage }) => {
        const api = await import(moduleUrl);
        const records: string[] = [];
        const receipt = { schema: "semio.space.home.directory-projection-receipt.v1", sessionBindingSha256: sourcePage.binding, authorizationGeneration: sourcePage.generation, throughSeqInclusive: sourcePage.through, receiptSha256: sourcePage.receipt };
        const terminal = (output: unknown) => ({ output, mutations: [], inverseGroup: { invocationId: "browser", mutations: [], inverseMutations: [] } });
        let release: (() => void) | undefined;
        const delayed = new Promise<void>((resolve) => { release = resolve; });
        const plugin = {
          pluginId: home.pluginId,
          createApp: async () => { records.push("create"); return 41; },
          destroyApp: async () => { records.push("destroy"); },
          handleAction: async (_instanceId: number, invocation: string) => {
            const parsed = JSON.parse(invocation);
            if (parsed.address.actionId === "setClient") {
              records.push(`identity:${parsed.arguments.clientId}:${parsed.arguments.clientName}`);
              return terminal(null);
            }
            records.push("page");
            return terminal(receipt);
          },
        };
        const app = { id: home.appId, controllerId: home.appId, modes: [{ id: "explore" }], defaultModeId: "explore", windowKinds: [{ id: "main", actions: [{ id: home.actionId }, { id: "setClient" }] }] };
        const posts: any[] = [];
        const owner = await api.openDirectoryHomeOwnerV1({ plugin, app, identity: { userId: "user-a", displayName: "Directory Browser A" }, instance: { instanceId: 41, viewState: { activeModeId: "explore" } }, baseUrl: "http://127.0.0.1:6070", bootstrapEpoch: 1, locale: "en", terminology: "native", beforeBootstrap: async () => { records.push("refresh-open"); }, post: (message: any) => posts.push(message) });
        const page = { kind: "directory-event-page", bootstrapEpoch: 1, canonicalJson: JSON.stringify({ schema: "semio.directory.event-page.v1", events: sourcePage.eventIds }), sessionBindingSha256: sourcePage.binding, authorizationGeneration: sourcePage.generation, afterSeqExclusive: sourcePage.after, throughSeqInclusive: sourcePage.through, hasMore: sourcePage.hasMore, receiptSha256: sourcePage.receipt };
        const applied = await api.applyDirectoryEventPageBootstrapV1(owner, page, (message: any) => posts.push(message), async () => { records.push("refresh-ack"); });
        await api.closeDirectoryHomeOwnerV1(owner, (message: any) => posts.push(message));
        const latePosts: any[] = [];
        const latePlugin = { ...plugin, handleAction: async (_instanceId: number, invocation: string) => {
          const parsed = JSON.parse(invocation);
          if (parsed.address.actionId === "setClient") return terminal(null);
          await delayed;
          return terminal(receipt);
        } };
        const lateOwner = await api.openDirectoryHomeOwnerV1({ plugin: latePlugin, app, identity: { userId: "user-b", displayName: "Directory Browser B" }, instance: { instanceId: 42, viewState: { activeModeId: "explore" } }, baseUrl: "http://127.0.0.1:6070", bootstrapEpoch: 2, locale: "de", terminology: "native", post: (message: any) => latePosts.push(message) });
        const late = api.applyDirectoryEventPageBootstrapV1(lateOwner, { ...page, bootstrapEpoch: 2 }, (message: any) => latePosts.push(message));
        await api.closeDirectoryHomeOwnerV1(lateOwner, (message: any) => latePosts.push(message));
        release!();
        const cancelled = await late;
        return { records, posts, applied, latePosts, cancelled };
      }, { moduleUrl, home: fixture.home, sourcePage: fixture.pages.initial! }),
      deadline,
    ]);
    const ack = result.posts.find((message: Record<string, any>) => message.kind === "directory-bootstrap-ack");
    if (JSON.stringify(result.records) !== JSON.stringify(["identity:user-a:Directory Browser A", "refresh-open", "page", "refresh-ack"]) || !ack || ack.receiptSha256 !== fixture.pages.initial!.receipt || ack.throughSeqInclusive !== fixture.pages.initial!.through || result.applied.state.kind !== "idle") throw new Error(`directory Home browser controller positive journey differed: ${JSON.stringify(result)}`);
    if (result.latePosts.some((message: Record<string, any>) => message.kind === "directory-bootstrap-ack") || result.cancelled.state.code !== "directory-bootstrap.cancelled") throw new Error("directory Home browser controller accepted a late terminal after close");
    console.log("directory-home-browser-runtime: Chromium actual same-visible-instance Hub identity, pre-open/pre-ACK refresh, ACK and late-cancel laws passed");
  } catch (error) {
    const diagnostics = browserDiagnostics.slice(-16).join("\n");
    throw new Error(`${error instanceof Error ? error.message : "directory Home browser controller failed"}${diagnostics ? `\nbrowser diagnostics:\n${diagnostics}` : ""}`);
  } finally {
    await browser?.close().catch(() => {});
    await runtimeServer?.stop(true);
  }
}

function assertDirectoryHomeBrowserComponentAttestation(repoRoot: string, fixture: DirectoryHomeBrowserProcessFixture): void {
  const moduleRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules", fixture.home.moduleDirectory);
  const manifest = JSON.parse(readFileSync(join(moduleRoot, "🔣️.json"), "utf8")) as Record<string, any>;
  const wasmPath = join(moduleRoot, "semio_s_plugin_space_component.core.wasm");
  const actual = createHash("sha256").update(readFileSync(wasmPath)).digest("hex");
  const expected = manifest.hashes?.coreWasmSha256;
  const app = manifest.manifest?.apps?.find((candidate: Record<string, any>) => candidate.id === fixture.home.appId);
  const failures = [];
  if (!app) failures.push(`missing-app=${fixture.home.appId}`);
  else {
    for (const actionId of ["setClient", fixture.home.actionId]) {
      if (!app.windowKinds?.some((window: Record<string, any>) => window.actions?.some((action: Record<string, any>) => action.id === actionId))) failures.push(`missing-action=${actionId}`);
    }
  }
  if (!/^[0-9a-f]{64}$/u.test(expected ?? "") || expected !== actual) failures.push(`core-wasm-sha256 expected=${expected ?? "<missing>"} actual=${actual}`);
  if (failures.length > 0) throw new Error(`directory Home browser process blocked before discovery: Space component attestation rejected: ${failures.join("; ")}`);
}

async function proveDirectoryHomeBrowserStaticWasmProcessRuntime(
  repoRoot: string,
  fixture: DirectoryHomeBrowserProcessFixture,
  source: Readonly<{ page: LiveDirectoryEventPageV1; userId: string; displayName: string; baseUrl: string }>,
): Promise<void> {
  const bundleModule = async (entrypoint: string): Promise<string> => {
    const bundle = await Bun.build({ entrypoints: [entrypoint], target: "browser", format: "esm", sourcemap: "none", define: { "import.meta.vitest": "undefined" } });
    if (!bundle.success || bundle.outputs.length !== 1) throw new Error(`directory Home real browser bundle failed: ${bundle.logs.map(String).join("; ") || "unexpected output closure"}`);
    return bundle.outputs[0]!.text();
  };
  const [controller, runtime] = await Promise.all([
    bundleModule(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx")),
    bundleModule(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx")),
  ]);
  const pluginRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules");
  const mime = (path: string): string => path.endsWith(".wasm") ? "application/wasm" : path.endsWith(".json") ? "application/json; charset=utf-8" : "text/javascript; charset=utf-8";
  const physicalPluginPath = (urlPath: string): string | undefined => {
    const prefix = "/🔌️plugin-modules/";
    const shardPrefix = "/plugin-modules/_shard/";
    const relativePath = urlPath.startsWith(prefix) ? urlPath.slice(prefix.length) : urlPath.startsWith(shardPrefix) ? join("🧵️shard", urlPath.slice(shardPrefix.length)) : undefined;
    if (!relativePath || relativePath.split("/").includes("..")) return undefined;
    const candidate = join(pluginRoot, relativePath);
    const escaped = relative(pluginRoot, candidate);
    return escaped.startsWith("..") || isAbsolute(escaped) || !existsSync(candidate) || !statSync(candidate).isFile() ? undefined : candidate;
  };
  const port = await freeLoopbackPort();
  const server = Bun.serve({
    hostname: "127.0.0.1",
    port,
    fetch(request): Response {
      const path = decodeURIComponent(new URL(request.url).pathname);
      if (path === "/controller.js") return new Response(controller, { headers: { "content-type": "text/javascript; charset=utf-8", "cache-control": "no-store" } });
      if (path === "/plugin-runtime.js") return new Response(runtime, { headers: { "content-type": "text/javascript; charset=utf-8", "cache-control": "no-store" } });
      if (path === "/__directory-home-process") return new Response("<!doctype html><meta charset=utf-8><title>Directory Home Process</title>", { headers: { "content-type": "text/html; charset=utf-8" } });
      const physical = physicalPluginPath(path);
      return physical ? new Response(readFileSync(physical), { headers: { "content-type": mime(physical), "cache-control": "no-store" } }) : new Response("not found", { status: 404 });
    },
  });
  let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
  const diagnostics: string[] = [];
  try {
    process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(repoRoot, "node_modules", ".cache", "ms-playwright");
    const { chromium } = await import("playwright");
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    page.on("console", (message) => diagnostics.push(`console:${message.type()}:${message.text()}`));
    page.on("pageerror", (error) => diagnostics.push(`pageerror:${error.message}`));
    page.on("requestfailed", (request) => diagnostics.push(`requestfailed:${request.url()}:${request.failure()?.errorText ?? "unknown"}`));
    await page.goto(`http://127.0.0.1:${port}/__directory-home-process`, { waitUntil: "domcontentloaded", timeout: fixture.limits.stepMs });
    const abort = AbortSignal.timeout(fixture.limits.journeyMs);
    const deadline = new Promise<never>((_, reject) => {
      const fail = () => reject(new Error("directory Home real browser process deadline exceeded"));
      if (abort.aborted) fail(); else abort.addEventListener("abort", fail, { once: true });
    });
    const result = await Promise.race([
      page.evaluate(async ({ home, source }) => {
        const controller = await import("/controller.js");
        const runtime = await import("/plugin-runtime.js");
        runtime.setPluginRuntimeActor(`user:${source.userId}#directory-home-process`);
        const plugin = await runtime.loadPluginModule(home.pluginId, `/🔌️plugin-modules/${home.moduleDirectory}/🌉️bridge.js`, AbortSignal.timeout(10_000));
        const app = plugin.manifest.apps.find((candidate: any) => candidate.id === home.appId);
        if (!app) throw new Error("directory Home real browser app unavailable after discovery");
        const posts: any[] = [];
        let owner: any;
        try {
          owner = await controller.openDirectoryHomeOwnerV1({ plugin, app, identity: { userId: source.userId, displayName: source.displayName }, baseUrl: source.baseUrl, bootstrapEpoch: 1, locale: "en", terminology: "native", post: (message: any) => posts.push(message) });
          const canonicalJson = JSON.stringify(source.page);
          const applied = await controller.applyDirectoryEventPageBootstrapV1(owner, {
            kind: "directory-event-page",
            bootstrapEpoch: 1,
            canonicalJson,
            sessionBindingSha256: source.page.sessionBindingSha256,
            authorizationGeneration: source.page.authorizationGeneration,
            afterSeqExclusive: source.page.afterSeqExclusive,
            throughSeqInclusive: source.page.throughSeqInclusive,
            hasMore: source.page.hasMore,
            receiptSha256: source.page.receiptSha256,
          }, (message: any) => posts.push(message));
          return { pluginId: plugin.manifest.pluginId, appId: app.id, action: app.windowKinds.some((window: any) => window.actions?.some((action: any) => action.id === home.actionId)), posts, applied };
        } finally {
          if (owner) await controller.closeDirectoryHomeOwnerV1(owner, (message: any) => posts.push(message));
          plugin.dispose();
        }
      }, { home: fixture.home, source }),
      deadline,
    ]);
    const ack = result.posts.find((message: Record<string, any>) => message.kind === "directory-bootstrap-ack");
    const expected = source.page;
    if (result.pluginId !== fixture.home.pluginId || result.appId !== fixture.home.appId || !result.action || result.applied.state.kind !== "idle" || !ack
      || ack.sessionBindingSha256 !== expected.sessionBindingSha256 || ack.authorizationGeneration !== expected.authorizationGeneration || ack.throughSeqInclusive !== expected.throughSeqInclusive || ack.receiptSha256 !== expected.receiptSha256) {
      throw new Error(`directory Home real browser terminal differed: ${JSON.stringify(result)}`);
    }
    console.log(`directory-home-browser-process-runtime: real-hub-page=1 static-dev-space-wasm=1 verified-activation=0 Chromium=1 through=${expected.throughSeqInclusive} receipt=${expected.receiptSha256}`);
  } catch (error) {
    const detail = diagnostics.slice(-24).join("\n");
    throw new Error(`${error instanceof Error ? error.message : "directory Home real browser process failed"}${detail ? `\nbrowser diagnostics:\n${detail}` : ""}`);
  } finally {
    await browser?.close().catch(() => {});
    await server.stop(true);
  }
}

async function proveDirectoryHomeBrowserProcessJourney(repoRoot: string, root: string, fixture: DirectoryHomeBrowserProcessFixture): Promise<void> {
  const binary = hubBinaryPath(repoRoot);
  if (!existsSync(binary)) throw new Error(`directory Home browser process blocked before discovery: os-hub binary absent at ${binary}`);
  const source = await proveDirectoryEventPageV1Process(repoRoot, root);
  assertDirectoryHomeBrowserComponentAttestation(repoRoot, fixture);
  await proveDirectoryHomeBrowserStaticWasmProcessRuntime(repoRoot, fixture, source);
}

type ScopedPresenceBrowserAuthorityCase = Readonly<{
  id: "a" | "b";
  scope: { readonly spaceId: string; readonly documentId: string };
  surfaceId: string;
  plan: Record<string, any>;
  installedTarget: Record<string, any>;
  socketGrant: Record<string, any>;
  openPath: string;
  grantPath: string;
  socketPath: string;
}>;

function scopedPresenceBrowserCases(fixture: BrowserDocumentOpenFixture): readonly [ScopedPresenceBrowserAuthorityCase, ScopedPresenceBrowserAuthorityCase] {
  const make = (id: "a" | "b", spaceId: string, surfaceId: string, actorByte: string): ScopedPresenceBrowserAuthorityCase => {
    const scope = { spaceId, documentId: fixture.intent.scope.documentId };
    const plan = structuredClone(fixture.plan) as Record<string, any>;
    const installedTarget = structuredClone(fixture.installedTarget) as Record<string, any>;
    const socketGrant = structuredClone(fixture.socketGrant) as Record<string, any>;
    plan.scope = scope;
    plan.surface.surfaceId = surfaceId;
    plan.surface.role = id === "a" ? "editor" : "viewer";
    plan.grant.write = id === "a";
    plan.expiresAtUnixMs = Date.now() + 30_000;
    installedTarget.scope = scope;
    installedTarget.surface.surfaceId = surfaceId;
    installedTarget.surface.role = id === "a" ? "editor" : "viewer";
    installedTarget.grant.write = id === "a";
    socketGrant.actorId = `hub.v1.${actorByte.repeat(64)}`;
    socketGrant.grant = `socket.v1.${actorByte.repeat(32)}.${actorByte.repeat(64)}`;
    socketGrant.expiresAtMs = Date.now() + 25_000;
    const root = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(scope.documentId)}`;
    return {
      id,
      scope,
      surfaceId,
      plan,
      installedTarget,
      socketGrant,
      openPath: `${root}/open-plan`,
      grantPath: `${root}/socket-grants`,
      socketPath: `${root}/socket/v1?surface=${encodeURIComponent(surfaceId)}`,
    };
  };
  return [
    make("a", fixture.expected.scopeIsolation.left.spaceId, "surface.gis.editor", "3"),
    make("b", fixture.expected.scopeIsolation.right.spaceId, "surface.gis.viewer", "4"),
  ];
}

/** 👥️ Runs the real browser Worker behind a mounted React Shell probe for interactive Chromium acceptance. */
async function serveScopedPresenceBrowserRuntime(repoRoot: string): Promise<void> {
  const fixture = await browserDocumentOpenFixture(repoRoot);
  const cases = scopedPresenceBrowserCases(fixture);
  const capability = `session.v1.${"a".repeat(32)}.${"b".repeat(64)}`;
  const effects = {
    a: { open: 0, exchange: 0, socket: 0, hello: 0, heartbeats: 0, closes: 0 },
    b: { open: 0, exchange: 0, socket: 0, hello: 0, heartbeats: 0, closes: 0 },
  };
  const authority = Bun.serve<{ id: "a" | "b" }>({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request, server): Promise<Response | undefined> {
      const url = new URL(request.url);
      const scopeCase = cases.find((row) => url.pathname === row.openPath || url.pathname === row.grantPath || `${url.pathname}${url.search}` === row.socketPath);
      if (!scopeCase) return new Response("", { status: 404 });
      if (`${url.pathname}${url.search}` === scopeCase.socketPath && request.method === "GET") {
        const protocols = (request.headers.get("sec-websocket-protocol") ?? "").split(",").map((value) => value.trim());
        if (protocols[0] !== "semio.socket.v1" || protocols[1] !== scopeCase.socketGrant.grant || request.headers.has("authorization")) return new Response("", { status: 401 });
        effects[scopeCase.id].socket += 1;
        return server.upgrade(request, { data: { id: scopeCase.id }, headers: { "Sec-WebSocket-Protocol": "semio.socket.v1" } }) ? undefined : new Response("", { status: 500 });
      }
      if (request.method !== "POST" || request.headers.get("authorization") !== `Bearer ${capability}`) return new Response("", { status: 401 });
      const raw = new Uint8Array(await request.arrayBuffer());
      try {
        if (raw.byteLength === 0 || raw.byteLength > 8 * 1024) return new Response("", { status: 413 });
        const input = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(raw)) as Record<string, any>;
        if (url.pathname === scopeCase.openPath) {
          if (input.schema !== fixture.intent.schema || input.version !== 1 || JSON.stringify(input.scope) !== JSON.stringify(scopeCase.scope) || input.requestedSurfaceId !== scopeCase.surfaceId) return new Response("", { status: 400 });
          effects[scopeCase.id].open += 1;
          return Response.json(scopeCase.plan, { headers: { "cache-control": "no-store" } });
        }
        if (url.pathname === scopeCase.grantPath) {
          if (input.schema !== "semio.hub.document-plan-socket-grant-intent/v1" || input.version !== 1 || input.planReceipt !== scopeCase.plan.receipt) return new Response("", { status: 400 });
          effects[scopeCase.id].exchange += 1;
          return Response.json(scopeCase.socketGrant, { headers: { "cache-control": "no-store" } });
        }
        return new Response("", { status: 404 });
      } finally {
        raw.fill(0);
      }
    },
    websocket: {
      message(socket, message): void {
        const scopeCase = cases.find((row) => row.id === socket.data.id)!;
        const decoded = decodeClientFrame(typeof message === "string" ? new TextEncoder().encode(message) : new Uint8Array(message));
        if (typeof decoded.frame === "string") return;
        if ("SocketHelloV1" in decoded.frame) {
          effects[scopeCase.id].hello += 1;
          const frontier = { document_id: scopeCase.scope.documentId, head_edit_ordinal: 0, head_edit_id: "", last_commit_seq: 0, chain_hash: new Array(32).fill(0) };
          socket.send(encodeServerFrame({ Welcome: { session_id: `presence-${scopeCase.id}`, resume_token: `presence-${scopeCase.id}-resume`, server_frontier: frontier, bootstrap: "None" } }, "command"));
          socket.send(encodeServerFrame({ Session: { actor: scopeCase.socketGrant.actorId, color: scopeCase.id === "a" ? 2 : 5 } }, "command"));
          return;
        }
        if (!("Presence" in decoded.frame)) return;
        effects[scopeCase.id].heartbeats += 1;
        const incoming = decodePresencePeer(new Uint8Array(decoded.frame.Presence.peer), [0]);
        const correct: ArtifactPresencePeer = {
          ...incoming,
          actor: scopeCase.socketGrant.actorId,
          userId: scopeCase.id === "a" ? "user-a" : "user-b",
          label: scopeCase.id === "a" ? "Ada" : "Berta",
          role: scopeCase.id === "a" ? "owner" : "viewer",
          connectedAtMs: scopeCase.id === "a" ? 101 : 202,
          color: scopeCase.id === "a" ? 2 : 5,
          surface: scopeCase.surfaceId,
          views: [],
        };
        const hostile: ArtifactPresencePeer = {
          ...correct,
          actor: `intruder-${scopeCase.id}`,
          userId: `intruder-${scopeCase.id}`,
          label: "Wrong Surface",
          surface: scopeCase.id === "a" ? cases[1].surfaceId : cases[0].surfaceId,
        };
        socket.send(encodeServerFrame({ Presence: { peers: [Array.from(encodePresencePeer(correct)), Array.from(encodePresencePeer(hostile))] } }, "preview"));
      },
      close(socket): void {
        effects[socket.data.id].closes += 1;
      },
    },
  });
  const hubOrigin = `http://127.0.0.1:${authority.port}`;
  const proof = randomBytes(32);
  const prior = {
    S_OS_PORT: process.env.S_OS_PORT,
    S_HUB_URL: process.env.S_HUB_URL,
    S_LOCAL_RELAY_URL: process.env.S_LOCAL_RELAY_URL,
    S_LOCAL_RELAY_SECRET: process.env.S_LOCAL_RELAY_SECRET,
    SEMIO_PLUGIN: process.env.SEMIO_PLUGIN,
    SEMIO_RENDERER: process.env.SEMIO_RENDERER,
  };
  let relay: LocalBrowserRelay | undefined;
  let vite: { close(): Promise<void>; listen(): Promise<void>; transformRequest(url: string): Promise<unknown> } | undefined;
  try {
    const uiPort = await freeLoopbackPort();
    const uiOrigin = `http://127.0.0.1:${uiPort}`;
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, { schema: "semio.hub.local-credential-envelope/v1", clientClass: "react-relay", capability }, proof);
    process.env.S_OS_PORT = String(uiPort);
    process.env.S_HUB_URL = hubOrigin;
    process.env.S_LOCAL_RELAY_URL = relay.url;
    process.env.S_LOCAL_RELAY_SECRET = relay.secret.toString("hex");
    process.env.SEMIO_PLUGIN = "s";
    process.env.SEMIO_RENDERER = "react";
    const componentPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/👥️presence-scope/🌐️browser/🟦️.tsx");
    const workerPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts");
    const shellEntryPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts");
    const browserConfig = {
      hubOrigin,
      workerUrl: `/@fs${workerPath}`,
      cases: cases.map((row) => ({ id: row.id, scope: row.scope, schema: fixture.expected.helloSchema, surfaceId: row.surfaceId, installedTarget: row.installedTarget })),
    };
    const refreshBrowserConfig = async (): Promise<Record<string, unknown>> => {
      if (!relay) throw new Error("scoped presence relay is unavailable");
      const now = Date.now();
      for (const row of cases) {
        row.plan.expiresAtUnixMs = now + 30_000;
        row.socketGrant.expiresAtMs = now + 25_000;
      }
      const binding = { port: Number(new URL(relay.url).port), secret: Buffer.from(relay.secret) };
      await relay.stop();
      const currentProof = randomBytes(32);
      const proofHex = currentProof.toString("hex");
      relay = startLocalBrowserRelay(hubOrigin, uiOrigin, { schema: "semio.hub.local-credential-envelope/v1", clientClass: "react-relay", capability }, currentProof, BROWSER_BROKER_PROOF_TTL_MS, binding);
      return { proof: proofHex, ...browserConfig };
    };
    const html = `<!doctype html><html><head><meta charset="utf-8"><title>Scoped presence browser shell</title><script type="module">import { injectIntoGlobalHook } from "/@react-refresh"; injectIntoGlobalHook(window); window.$RefreshReg$ = () => {}; window.$RefreshSig$ = () => (type) => type;</script><script type="module" src="/@vite/client"></script></head><body><div id="root"></div><script type="module">const root = document.querySelector("#root"); try { const module = await import(${JSON.stringify(`/@fs${componentPath}`)}); const response = await fetch("/__scoped-presence/config"); if (!response.ok) throw new Error(\`config status \${response.status}\`); module.mountScopedPresenceBrowserShellV1(root, await response.json()); } catch (error) { root.textContent = \`[DEBUG] scoped-presence mount failed: \${error instanceof Error ? error.message : String(error)}\`; console.error(root.textContent); }</script></body></html>`;
    const { createServer: createViteServer } = await import("vite");
    vite = await createViteServer({
      configFile: join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts"),
      server: { host: "127.0.0.1", port: uiPort, strictPort: true },
      clearScreen: false,
      plugins: [{
        name: "semio-scoped-presence-browser-runtime",
        configureServer(server) {
          server.middlewares.use(async (request, response, next) => {
            if (request.url === "/__scoped-presence/config") {
              response.setHeader("content-type", "application/json; charset=utf-8");
              response.end(JSON.stringify(await refreshBrowserConfig()));
              return;
            }
            if (request.url === "/__scoped-presence/effects") {
              response.setHeader("content-type", "application/json; charset=utf-8");
              response.end(JSON.stringify(effects));
              return;
            }
            if (request.url === "/__scoped-presence") {
              response.setHeader("content-type", "text/html; charset=utf-8");
              response.end(html);
              return;
            }
            next();
          });
        },
      }],
    });
    await vite.listen();
    if (await vite.transformRequest(`/@fs${componentPath}`) === null || await vite.transformRequest(`/@fs${workerPath}`) === null || await vite.transformRequest(`/@fs${shellEntryPath}`) === null) throw new Error("scoped presence browser or production shell module did not transform");
    console.log(`scoped-presence-browser-serve: url=${uiOrigin}/__scoped-presence effects=${uiOrigin}/__scoped-presence/effects shell=${uiOrigin}/`);
    await new Promise<void>((resolveStop) => {
      const stop = (): void => resolveStop();
      process.once("SIGINT", stop);
      process.once("SIGTERM", stop);
    });
  } finally {
    await vite?.close().catch(() => {});
    await relay?.stop().catch(() => {});
    await authority.stop(true);
    for (const [key, value] of Object.entries(prior)) {
      if (value === undefined) delete process.env[key];
      else process.env[key] = value;
    }
  }
}

class ScopedPresenceBrowserServeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("scoped-presence-browser-serve accepts no arguments");
    await serveScopedPresenceBrowserRuntime(this.repoRoot);
  }
}

class DirectoryHomeBrowserProcessCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "runtime", "process"].includes(phase)) throw new Error("directory-home-browser-process-check accepts source, runtime, or process");
    const fixture = await proveDirectoryHomeBrowserProcessSource(this.repoRoot);
    if (phase === "runtime" || phase === "process") await proveDirectoryHomeBrowserControllerRuntime(this.repoRoot, fixture);
    if (phase === "process") {
      runCmd("cargo", ["build", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
      await proveDirectoryHomeBrowserProcessJourney(this.repoRoot, this.root, fixture);
    }
    console.log(`directory-home-browser-process-check: phase=${phase} traces=${fixture.traces.length} hostile=${fixture.hostile.length}`);
  }
}

/** 🌎️ Proves the receipt producer against a restarted real SQLite hub and two independent local identities. */
async function proveDirectoryEventPageV1Process(repoRoot: string, root: string): Promise<Readonly<{ page: LiveDirectoryEventPageV1; userId: string; displayName: string; baseUrl: string }>> {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot || !isAbsolute(artifactRoot)) throw new Error("directory event page process requires an absolute ticket-local SEMIO_TEST_ARTIFACT_DIR");
  const dataRoot = join(artifactRoot, `sqlite-process-${randomBytes(8).toString("hex")}`);
  mkdirSync(dataRoot, { recursive: true });
  const profiles: readonly LocalProfile[] = [
    { profileId: "event-page-a", subject: "event-page-process-a", displayName: "Event Page A", allowedClientClasses: ["native"] },
    { profileId: "event-page-b", subject: "event-page-process-b", displayName: "Event Page B", allowedClientClasses: ["native"] },
  ];
  const deadline = Date.now() + 90_000;
  const checkpoint = (): void => { if (Date.now() >= deadline) throw new Error("directory event page process deadline exceeded"); };
  let first: LocalHubRun | undefined;
  let second: LocalHubRun | undefined;
  let envelopeA: Record<string, any> | undefined;
  let envelopeB: Record<string, any> | undefined;
  let restartedA: Record<string, any> | undefined;
  try {
    first = await startLocalHub(repoRoot, root, profiles, { capture: true, isolatedSecuritySmoke: true, dataDir: dataRoot });
    await waitForReadiness(first, true);
    envelopeA = await issueLocalCredential(first, profiles[0]!.profileId, "native", 2);
    envelopeB = await issueLocalCredential(first, profiles[1]!.profileId, "native", 3);
    const userA = await liveDirectoryEventPageUser(first, envelopeA);
    const userB = await liveDirectoryEventPageUser(first, envelopeB);
    const baselinePage = await fetchLiveDirectoryEventPage(first, envelopeA, userA, 0);
    const baseline = baselinePage.throughSeqInclusive;
    const createdA = await submitLiveDirectoryCommand(first, envelopeA, { kind: "create-space", name: "Visible A 0", spaceKind: "studio", visibility: "private" });
    const spaceA = createdLiveDirectorySpace(createdA);
    const createdB = await submitLiveDirectoryCommand(first, envelopeB, { kind: "create-space", name: "Hidden B 0", spaceKind: "studio", visibility: "private" });
    const spaceB = createdLiveDirectorySpace(createdB);
    const visibleSeqs = [
      ...createdA,
      ...(await submitLiveDirectoryCommand(first, envelopeA, { kind: "rename-space", spaceId: spaceA, name: "Visible A 1" })),
      ...(await submitLiveDirectoryCommand(first, envelopeB, { kind: "rename-space", spaceId: spaceB, name: "Hidden B 1" })),
      ...(await submitLiveDirectoryCommand(first, envelopeA, { kind: "rename-space", spaceId: spaceA, name: "Visible A 2" })),
      ...(await submitLiveDirectoryCommand(first, envelopeB, { kind: "rename-space", spaceId: spaceB, name: "Hidden B 2" })),
    ].filter((event) => event.spaceId === spaceA).map((event) => event.seq);
    const holes = await fetchLiveDirectoryEventPage(first, envelopeA, userA, baseline);
    const holeSource = JSON.stringify(holes);
    if (JSON.stringify(holes.events.map((event) => event.seq)) !== JSON.stringify(visibleSeqs) || holeSource.includes(spaceB) || holeSource.includes(userB.userId) || holeSource.includes("Hidden B")) {
      throw new Error("directory event page process raw-hole visibility differed from current membership");
    }

    const prefixAfter = holes.throughSeqInclusive;
    const firstLarge = (await submitLiveDirectoryCommand(first, envelopeA, { kind: "rename-space", spaceId: spaceA, name: "a".repeat(32 * 1024) }))[0]!.seq;
    const secondLarge = (await submitLiveDirectoryCommand(first, envelopeA, { kind: "rename-space", spaceId: spaceA, name: "b".repeat(32 * 1024) }))[0]!.seq;
    const prefix = await fetchLiveDirectoryEventPage(first, envelopeA, userA, prefixAfter);
    if (JSON.stringify(prefix.events.map((event) => event.seq)) !== JSON.stringify([firstLarge]) || prefix.throughSeqInclusive !== firstLarge || !prefix.hasMore) {
      throw new Error("directory event page process byte prefix skipped or duplicated a visible row");
    }
    const continuation = await fetchLiveDirectoryEventPage(first, envelopeA, userA, prefix.throughSeqInclusive);
    if (JSON.stringify(continuation.events.map((event) => event.seq)) !== JSON.stringify([secondLarge]) || continuation.throughSeqInclusive !== secondLarge) {
      throw new Error("directory event page process continuation did not return the deferred visible row exactly once");
    }

    let lastHidden = secondLarge;
    for (let index = 0; index < 128; index += 1) {
      checkpoint();
      lastHidden = (await submitLiveDirectoryCommand(first, envelopeB, { kind: "rename-space", spaceId: spaceB, name: `Hidden B saturated ${index}` }))[0]!.seq;
    }
    const saturated = await fetchLiveDirectoryEventPage(first, envelopeA, userA, secondLarge);
    const saturatedSource = JSON.stringify(saturated);
    if (saturated.events.length !== 0 || saturated.throughSeqInclusive !== lastHidden || !saturated.hasMore || saturatedSource.includes(spaceB) || saturatedSource.includes(userB.userId) || saturatedSource.includes("Hidden B saturated")) {
      throw new Error("directory event page process did not advance one saturated raw-hidden scan exactly");
    }

    const revoked = await fetch(`http://127.0.0.1:${first.port}/auth/sessions/me`, { method: "DELETE", headers: { authorization: `Bearer ${envelopeB.capability}` }, signal: AbortSignal.timeout(2_000) });
    const stale = await fetch(`http://127.0.0.1:${first.port}/directory/event-page/v1?after=0`, { headers: { authorization: `Bearer ${envelopeB.capability}` }, signal: AbortSignal.timeout(2_000) });
    if (revoked.status !== 204 || stale.status !== 401 || (await stale.arrayBuffer()).byteLength !== 0) throw new Error("directory event page process stale bearer denial was not empty and terminal");
    const priorBinding = continuation.sessionBindingSha256;
    envelopeB.capability = "";
    await finishLocalHub(first);
    first = undefined;

    second = await startLocalHub(repoRoot, root, profiles, { capture: true, isolatedSecuritySmoke: true, dataDir: dataRoot });
    await waitForReadiness(second, true);
    restartedA = await issueLocalCredential(second, profiles[0]!.profileId, "native", 2);
    const restartedUser = await liveDirectoryEventPageUser(second, restartedA);
    const persisted = await fetchLiveDirectoryEventPage(second, restartedA, restartedUser, firstLarge);
    const persistedSource = JSON.stringify(persisted);
    if (persisted.events[0]?.seq !== secondLarge || persisted.throughSeqInclusive <= secondLarge || !persisted.hasMore || persisted.sessionBindingSha256 === priorBinding || persistedSource.includes(spaceB) || persistedSource.includes(userB.userId) || persistedSource.includes("Hidden B saturated")) {
      throw new Error("directory event page process restart lost durable visible/raw frontier or retained a stale session binding");
    }
    console.log("directory-event-page-v1-process: real SQLite two-user holes, prefix, stale bearer, and restart receipt passed");
    return { page: persisted, userId: restartedUser.userId, displayName: restartedUser.displayName, baseUrl: `http://127.0.0.1:${second.port}` };
  } catch (error) {
    const diagnostics = `${first?.output() ?? ""}\n${second?.output() ?? ""}`.slice(-4_096);
    throw new Error(`${error instanceof Error ? error.message : "directory event page process failed"}${diagnostics ? `\nhub diagnostics:\n${diagnostics}` : ""}`);
  } finally {
    if (envelopeA) envelopeA.capability = "";
    if (envelopeB) envelopeB.capability = "";
    if (restartedA) restartedA.capability = "";
    if (first) await finishLocalHub(first);
    if (second) await finishLocalHub(second);
  }
}

/** 🔎️ Finds the one invite roster anywhere in a directory administration projection, accepting both
 * a bare `invites` array and the current bounded `invites.rows` window shape. */
function liveDirectoryInviteRows(value: unknown): readonly Record<string, any>[] | undefined {
  if (Array.isArray(value)) {
    for (const entry of value) {
      const found = liveDirectoryInviteRows(entry);
      if (found) return found;
    }
    return undefined;
  }
  if (value === null || typeof value !== "object") return undefined;
  for (const [key, child] of Object.entries(value as Record<string, unknown>)) {
    if (key === "invites") {
      if (Array.isArray(child)) return child as readonly Record<string, any>[];
      const rows = child !== null && typeof child === "object" ? (child as Record<string, unknown>).rows : undefined;
      if (Array.isArray(rows)) return rows as readonly Record<string, any>[];
    }
    const found = liveDirectoryInviteRows(child);
    if (found) return found;
  }
  return undefined;
}

/** 🧾️ Two real authenticated sessions against a real SQLite hub: the author issues an invite through
 * the V1 endpoint, exactly one response carries the capability, a disconnect-then-resolve of the same
 * id is redacted and mints nothing, a spectator is generically forbidden, and a revoked author cannot
 * retrieve a previous receipt. Hub process evidence only — it claims no rendered administration UI. */
async function proveDirectoryCommandReceiptV1Process(repoRoot: string, root: string): Promise<void> {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot || !isAbsolute(artifactRoot)) throw new Error("directory command receipt process requires an absolute ticket-local SEMIO_TEST_ARTIFACT_DIR");
  const dataRoot = join(artifactRoot, `command-receipt-${randomBytes(8).toString("hex")}`);
  mkdirSync(dataRoot, { recursive: true });
  const profiles: readonly LocalProfile[] = [
    { profileId: "command-receipt-author", subject: "command-receipt-process-author", displayName: "Command Receipt Author", allowedClientClasses: ["native"] },
    { profileId: "command-receipt-spectator", subject: "command-receipt-process-spectator", displayName: "Command Receipt Spectator", allowedClientClasses: ["native"] },
  ];
  let run: LocalHubRun | undefined;
  let author: Record<string, any> | undefined;
  let spectator: Record<string, any> | undefined;
  try {
    run = await startLocalHub(repoRoot, root, profiles, { capture: true, isolatedSecuritySmoke: true, dataDir: dataRoot });
    await waitForReadiness(run, true);
    author = await issueLocalCredential(run, profiles[0]!.profileId, "native", 2);
    spectator = await issueLocalCredential(run, profiles[1]!.profileId, "native", 3);
    const spectatorUser = await liveDirectoryEventPageUser(run, spectator);
    const spaceId = createdLiveDirectorySpace(await submitLiveDirectoryCommand(run, author, { kind: "create-space", name: "Command Receipt Space", spaceKind: "studio", visibility: "private" }));
    await submitLiveDirectoryCommand(run, author, { kind: "upsert-member", spaceId, email: spectatorUser.email, role: "spectator" });

    const inviteCommand = { kind: "create-invite", spaceId, role: "spectator", ttlSecs: 3600 };
    const requestId = liveDirectoryCommandRequestId();
    const first = await postLiveDirectoryCommand(run, author.capability, requestId, inviteCommand);
    const firstReceipt = first.status === 202 ? JSON.parse(first.text) as Record<string, any> : undefined;
    const token = firstReceipt?.result?.inviteToken;
    if (!firstReceipt || firstReceipt.outcome !== "accepted" || firstReceipt.requestId !== requestId || typeof token !== "string" || token.length === 0) {
      throw new Error(`directory command receipt process first invite did not carry exactly one capability: ${first.status}`);
    }

    const retry = await postLiveDirectoryCommand(run, author.capability, requestId, inviteCommand);
    const retryReceipt = retry.status === 202 ? JSON.parse(retry.text) as Record<string, any> : undefined;
    if (!retryReceipt || retryReceipt.outcome !== "secret-undeliverable" || retryReceipt.result?.kind !== "none" || retry.text.includes(token) || (retryReceipt.events ?? []).length !== 0) {
      throw new Error(`directory command receipt process same-id resolution was not redacted: ${retry.status}`);
    }

    const detail = await fetch(`http://127.0.0.1:${run.port}/directory/spaces/${spaceId}`, { headers: { authorization: `Bearer ${author.capability}` }, signal: AbortSignal.timeout(5_000) });
    const detailText = await detail.text();
    const invites = liveDirectoryInviteRows(JSON.parse(detailText));
    if (!invites || invites.length !== 1 || detailText.includes(token)) throw new Error("directory command receipt process retry minted a duplicate invitation or persisted its plaintext");

    const conflict = await postLiveDirectoryCommand(run, author.capability, requestId, { kind: "rename-space", spaceId, name: "Substituted" });
    if (conflict.status !== 409) throw new Error(`directory command receipt process equal id with a different command was not a conflict: ${conflict.status}`);

    const forbidden = await postLiveDirectoryCommand(run, spectator.capability, liveDirectoryCommandRequestId(), inviteCommand);
    if (forbidden.status !== 403 || forbidden.text.includes(token) || forbidden.text.includes(spaceId)) throw new Error(`directory command receipt process spectator denial was not generic: ${forbidden.status}`);

    const oversize = await postLiveDirectoryCommand(run, author.capability, liveDirectoryCommandRequestId(), inviteCommand, JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: liveDirectoryCommandRequestId(), command: { kind: "rename-space", spaceId, name: "x".repeat(9 * 1024) } }));
    if (oversize.status !== 413 && oversize.status !== 400) throw new Error(`directory command receipt process did not bound an oversize request: ${oversize.status}`);

    await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, { method: "DELETE", headers: { authorization: `Bearer ${author.capability}` }, signal: AbortSignal.timeout(5_000) });
    const revoked = await postLiveDirectoryCommand(run, author.capability, requestId, inviteCommand);
    if (revoked.status !== 401 || revoked.text.includes(token)) throw new Error(`directory command receipt process revoked author retrieved a stored completion: ${revoked.status}`);
    console.log("directory-command-receipt-process: one-shot capability, redacted same-id resolution, conflict, spectator denial, byte ceiling, and revoked-author denial passed");
  } catch (error) {
    const diagnostics = `${run?.output() ?? ""}`.slice(-4_096);
    throw new Error(`${error instanceof Error ? error.message : "directory command receipt process failed"}${diagnostics ? `\nhub diagnostics:\n${diagnostics}` : ""}`);
  } finally {
    if (author) author.capability = "";
    if (spectator) spectator.capability = "";
    if (run) await finishLocalHub(run);
  }
}

/** 🧾️ Language-agnostic vectors for the closed directory command request/receipt wire. */
type DirectoryCommandReceiptFixture = {
  readonly schema: "semio.hub.directory-command-receipt/v1";
  readonly limits: { readonly requestBytes: 8192; readonly receiptBytes: 65536; readonly maxEvents: 4; readonly inviteTokenBytes: 256; readonly requestIdLen: 32 };
  readonly requests: readonly { readonly name: string; readonly requestId: string; readonly command: DirectoryCommand; readonly canonical: string; readonly canonicalBytes: number; readonly commandSha256: string }[];
  readonly receipts: readonly { readonly name: string; readonly requestName: string; readonly outcome: DirectoryCommandOutcomeV1; readonly canonical: string; readonly canonicalBytes: number; readonly receiptSha256: string; readonly receipt: DirectoryCommandReceiptV1 }[];
  readonly rejectedRequests: readonly { readonly name: string; readonly source: string; readonly code: "invalid" | "too-large" }[];
  readonly rejectedReceipts: readonly { readonly name: string; readonly requestName: string; readonly source: string; readonly code: "invalid" | "too-large" }[];
  readonly transport: {
    readonly capacity: 64;
    readonly statusCodes: readonly { readonly status: number; readonly code: DirectoryCommandErrorCodeV1 }[];
    readonly transientCodes: readonly DirectoryCommandErrorCodeV1[];
    readonly terminalCodes: readonly DirectoryCommandErrorCodeV1[];
    readonly traces: readonly { readonly name: string; readonly expected: Record<string, unknown> }[];
  };
};

/** 🧾️ Proves the closed command wire from neutral vectors with two independent oracles: AJV over
 * the published fixture schema, and a Node `createHash` recomputation of every canonical digest.
 * The repository's own TypeScript parser is exercised as a third implementation, never as the
 * oracle, and the hub/os fixture copies must stay byte-identical. */
async function proveDirectoryCommandReceiptV1(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/🧪️fixtures/📇️directory/🧾️command-receipt-v1");
  const source = readFileSync(join(root, "🔣️.json"), "utf8");
  const mirror = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧾️command-receipt-v1.json"), "utf8");
  if (source !== mirror) throw new Error("directory command receipt fixture drifted between the hub and os trees");
  const fixture = JSON.parse(source) as DirectoryCommandReceiptFixture;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`directory command receipt fixture: ${JSON.stringify(validate.errors)}`);
  const digest = (text: string): string => createHash("sha256").update(Buffer.from(text, "utf8")).digest("hex");
  const bytes = (text: string): number => Buffer.byteLength(text, "utf8");
  let checks = 1;

  const parsedRequests = new Map<string, DirectoryCommandRequestV1>();
  for (const request of fixture.requests) {
    const canonical = JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: request.requestId, command: request.command });
    if (canonical !== request.canonical || bytes(canonical) !== request.canonicalBytes || request.canonicalBytes > fixture.limits.requestBytes) throw new Error(`directory command request '${request.name}' is not canonical within the request ceiling`);
    if (digest(JSON.stringify(request.command)) !== request.commandSha256) throw new Error(`directory command request '${request.name}' digest is not byte-exact`);
    const parsed = parseDirectoryCommandRequestV1(request.canonical);
    if (directoryCommandRequestJson(parsed) !== request.canonical || (await directoryCommandSha256(parsed.command)) !== request.commandSha256) throw new Error(`directory command request '${request.name}' does not round-trip`);
    if (directoryCommandRequestJson(sealDirectoryCommandRequestV1(request.requestId, request.command)) !== request.canonical) throw new Error(`directory command request '${request.name}' seal is not byte-identical`);
    parsedRequests.set(request.name, parsed);
    checks += 4;
  }
  const substituted = fixture.requests.find((request) => request.name === "substituted-command");
  const original = fixture.requests.find((request) => request.name === "create-invite");
  if (!substituted || !original || substituted.requestId !== original.requestId || substituted.commandSha256 === original.commandSha256) throw new Error("directory command fixture lacks an equal-id/different-digest conflict vector");
  checks += 1;

  const inviteTokens = new Set<string>();
  for (const receipt of fixture.receipts) {
    const request = parsedRequests.get(receipt.requestName);
    if (!request) throw new Error(`directory command receipt '${receipt.name}' names no request vector`);
    const { receiptSha256, ...unsigned } = receipt.receipt;
    if (digest(JSON.stringify(unsigned)) !== receipt.receiptSha256 || receiptSha256 !== receipt.receiptSha256) throw new Error(`directory command receipt '${receipt.name}' digest is not byte-exact`);
    if (JSON.stringify(receipt.receipt) !== receipt.canonical || bytes(receipt.canonical) !== receipt.canonicalBytes || receipt.canonicalBytes > fixture.limits.receiptBytes) throw new Error(`directory command receipt '${receipt.name}' is not canonical within the receipt ceiling`);
    if (receipt.receipt.events.length > fixture.limits.maxEvents) throw new Error(`directory command receipt '${receipt.name}' exceeds the durable event ceiling`);
    let previous = 0;
    for (const event of receipt.receipt.events) {
      if (event.seq <= previous) throw new Error(`directory command receipt '${receipt.name}' repeats or reorders a durable sequence`);
      previous = event.seq;
    }
    if (receipt.outcome !== "accepted" && (receipt.receipt.events.length > 0 || receipt.receipt.result.kind !== "none")) throw new Error(`directory command receipt '${receipt.name}' leaks a result through a redacted outcome`);
    if (receipt.receipt.result.kind === "invite") {
      if (receipt.outcome !== "accepted" || bytes(receipt.receipt.result.inviteToken) > fixture.limits.inviteTokenBytes) throw new Error(`directory command receipt '${receipt.name}' carries an inadmissible capability`);
      inviteTokens.add(receipt.receipt.result.inviteToken);
    }
    const parsed = await parseDirectoryCommandReceiptV1(receipt.canonical, request);
    if (parsed.receiptSha256 !== receipt.receiptSha256 || parsed.commandSha256 !== fixture.requests.find((entry) => entry.name === receipt.requestName)?.commandSha256) throw new Error(`directory command receipt '${receipt.name}' does not round-trip`);
    checks += 6;
  }
  if (inviteTokens.size === 0) throw new Error("directory command fixture proves no live invite delivery");
  for (const receipt of fixture.receipts.filter((entry) => entry.outcome !== "accepted")) {
    for (const token of inviteTokens) {
      if (receipt.canonical.includes(token)) throw new Error(`directory command receipt '${receipt.name}' replays a one-shot capability`);
      checks += 1;
    }
  }

  for (const rejected of fixture.rejectedRequests) {
    let admitted = true;
    try {
      parseDirectoryCommandRequestV1(rejected.source);
    } catch {
      admitted = false;
    }
    if (admitted) throw new Error(`directory command request '${rejected.name}' was admitted`);
    if (rejected.code === "too-large" && bytes(rejected.source) <= fixture.limits.requestBytes) throw new Error(`directory command request '${rejected.name}' is not actually over the request ceiling`);
    checks += 1;
  }
  for (const rejected of fixture.rejectedReceipts) {
    const request = parsedRequests.get(rejected.requestName);
    if (!request) throw new Error(`directory command receipt '${rejected.name}' names no request vector`);
    let admitted = true;
    try {
      await parseDirectoryCommandReceiptV1(rejected.source, request);
    } catch {
      admitted = false;
    }
    if (admitted) throw new Error(`directory command receipt '${rejected.name}' was admitted`);
    if (rejected.code === "too-large" && bytes(rejected.source) <= fixture.limits.receiptBytes) throw new Error(`directory command receipt '${rejected.name}' is not actually over the receipt ceiling`);
    checks += 1;
  }

  for (const mapping of fixture.transport.statusCodes) {
    if (directoryCommandErrorFromStatus(mapping.status) !== mapping.code) throw new Error(`directory command status ${mapping.status} does not map to '${mapping.code}'`);
    checks += 1;
  }
  for (const code of fixture.transport.transientCodes) {
    if (!directoryCommandErrorIsTransient(code)) throw new Error(`directory command code '${code}' is not transient`);
    checks += 1;
  }
  for (const code of fixture.transport.terminalCodes) {
    if (directoryCommandErrorIsTransient(code)) throw new Error(`directory command code '${code}' must be terminal`);
    checks += 1;
  }
  if (fixture.transport.capacity !== 64 || fixture.transport.traces.length < 6) throw new Error("directory command transport traces are incomplete");
  checks += 1;
  return checks;
}

class DirectoryCommandReceiptCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native", "process"].includes(phase)) throw new Error("directory-command-receipt-check accepts source, native, or process");
    const checks = await proveDirectoryCommandReceiptV1(this.repoRoot);
    if (phase === "native" || phase === "process") {
      const laws = [
        "directory_command_receipt_v1_route_is_request_idempotent_for_concurrent_identical_ids",
        "directory_command_receipt_v1_route_denies_cross_user_spectator_and_digest_substitution",
        "directory_command_receipt_v1_route_bounds_request_and_receipt_bytes",
        "directory_command_receipt_v1_store_resolves_a_lost_reply_and_survives_restart",
      ];
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-hub", target: { kind: "bin", name: "os-hub" }, cargoArgs: ["--all-features"], laws }],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) { console.log(`directory-command-receipt-${phase} ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`directory-command-receipt-${phase}-receipt: ${JSON.stringify(receipt)}`);
      if (phase === "process") {
        runCmd("cargo", ["build", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
        await proveDirectoryCommandReceiptV1Process(this.repoRoot, this.root);
      }
    }
    console.log(`directory-command-receipt-check: checks=${checks} phase=${phase}`);
  }
}

class DirectoryEventPageV1CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native", "process"].includes(phase)) throw new Error("directory-event-page-v1-check accepts source, native, or process");
    const checks = await proveDirectoryEventPageRouteV1(this.repoRoot);
    if (phase === "native" || phase === "process") {
      const laws = [
        "directory_event_page_v1_route_scans_raw_holes_bounds_canonical_receipt_and_visibility",
        "directory_event_page_v1_route_revalidates_session_generation_after_read_before_response",
        "directory_event_page_v1_route_stops_at_canonical_byte_prefix_without_skipping_visible_seq",
        "directory_event_page_v1_append_admission_is_transactional_for_sqlite_postgres_and_neo4j",
        "directory_event_page_v1_route_rejects_noncanonical_query_and_stale_bearer_without_body",
      ];
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [
          { package: "semio-hub", target: { kind: "bin", name: "os-hub" }, cargoArgs: ["--all-features"], laws },
          ...(phase === "native" ? [{
            package: "semio-hub",
            target: { kind: "lib" as const, name: "semio_hub" },
            cargoArgs: ["--all-features"],
            laws: [
              "directory::sqlite::tests::directory_event_page_v1_append_admission_is_transactional_sqlite",
              "directory::postgres::tests::directory_event_page_v1_append_admission_is_transactional_postgres",
              "directory::neo4j::tests::directory_event_page_v1_append_admission_is_transactional_neo4j",
            ],
          }] : []),
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) { console.log(`directory-event-page-v1-${phase} ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`directory-event-page-v1-${phase}-receipt: ${JSON.stringify(receipt)}`);
      if (phase === "process") {
        runCmd("cargo", ["build", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
        await proveDirectoryEventPageV1Process(this.repoRoot, this.root);
        runCmd("cargo", ["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
      }
    }
    console.log(`directory-event-page-v1-check: checks=${checks} phase=${phase}`);
  }
}


/** 🧯️ Structural fence: the worker's one terminal transition must erase EVERY retained
 * administration field — the page bytes, the receipt, and the invite capability — before it settles
 * a phase. Written against the extracted function body rather than a contiguous literal so a sibling
 * lane may add retained fields (the invite transfer epoch did) without silently disarming the law. */
function directoryAdministrationTerminateErases(browser: string): boolean {
  const start = browser.indexOf("function terminateDirectoryAdministration(");
  if (start < 0) return false;
  const body = browser.slice(start, browser.indexOf("\n}", start));
  const erased = ["canonicalJson", "receiptSha256", "outcome", "inviteToken", "requestId"];
  return erased.every((field) => body.includes(`operation.${field} = null;`))
    && body.includes("operation.phase = phase;")
    && body.includes("operation.abort.abort(")
    && body.indexOf("operation.phase = phase;") > body.indexOf("operation.inviteToken = null;");
}

/** 🏛️ Replays the bounded space-administration page contract without using the Rust implementation. */
type DirectorySpaceAdministrationFixture = {
  readonly schema: "semio.hub.directory-space-administration-page/v1";
  readonly limits: { readonly windowRows: 64; readonly pageBytes: 49152; readonly cursorBytes: 1024; readonly safeInteger: 9007199254740991 };
  readonly session: { readonly sessionId: string; readonly userId: string; readonly authorizationGeneration: number; readonly expiresAt: number; readonly spaceId: string; readonly bindingSha256: string };
  readonly space: Record<string, unknown>;
  readonly members: readonly { readonly userId: string; readonly email: string; readonly displayName: string; readonly role: string; readonly owner: boolean }[];
  readonly invites: readonly { readonly inviteId: string; readonly role: string; readonly createdAtMs: number; readonly expiresAtMs: number; readonly revoked: boolean; readonly accepted: boolean }[];
  readonly vectors: readonly { readonly name: string; readonly access: "author" | "member" | "public"; readonly expected: { readonly hasMembers: boolean; readonly hasInvites: boolean; readonly hasCapabilities: boolean; readonly memberRows: number; readonly inviteRows: number } }[];
  readonly cursorCases: readonly { readonly query: string; readonly status: number; readonly reads: number }[];
  readonly hostiles: readonly string[];
};

async function proveDirectorySpaceAdministrationPageV1(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/🧪️fixtures/📇️directory/🏘️space-administration-page-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as DirectorySpaceAdministrationFixture;
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`space administration fixture: ${JSON.stringify(validate.errors)}`);
  const u32be = (value: number): Buffer => { const bytes = Buffer.alloc(4); bytes.writeUInt32BE(value); return bytes; };
  const u64be = (value: number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
  const i64be = (value: number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigInt64BE(BigInt(value)); return bytes; };
  const sessionId = Buffer.from(fixture.session.sessionId, "utf8");
  const userId = Buffer.from(fixture.session.userId, "utf8");
  const spaceId = Buffer.from(fixture.session.spaceId, "utf8");
  const binding = createHash("sha256")
    .update(Buffer.concat([
      Buffer.from("semio/hub/directory-space-administration/session-binding/v1\0"),
      u32be(sessionId.length), sessionId,
      u32be(userId.length), userId,
      u64be(fixture.session.authorizationGeneration),
      i64be(fixture.session.expiresAt),
      u32be(spaceId.length), spaceId,
    ]))
    .digest("hex");
  if (binding !== fixture.session.bindingSha256) throw new Error("space administration session binding is not byte-exact");
  const alternateSession = Buffer.from("session-admin-0", "utf8");
  const alternateUser = Buffer.from("1user-admin-01", "utf8");
  if (Buffer.concat([sessionId, userId]).compare(Buffer.concat([alternateSession, alternateUser])) !== 0) throw new Error("binding ambiguity fixture drifted");
  const alternateBinding = createHash("sha256")
    .update(Buffer.concat([Buffer.from("semio/hub/directory-space-administration/session-binding/v1\0"), u32be(alternateSession.length), alternateSession, u32be(alternateUser.length), alternateUser, u64be(fixture.session.authorizationGeneration), i64be(fixture.session.expiresAt), u32be(spaceId.length), spaceId]))
    .digest("hex");
  if (alternateBinding === binding) throw new Error("length-prefixed session binding aliased concatenated identities");

  const capabilities = { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true };
  const memberRow = (index: number) => (index < fixture.members.length ? fixture.members[index]! : { userId: `user-${String(index).padStart(4, "0")}`, email: `u${index}@example.invalid`, displayName: `U${index}`, role: "spectator" as const, owner: false });
  const inviteRow = (index: number) => (index < fixture.invites.length ? fixture.invites[index]! : { inviteId: `invite-${String(1000 - index).padStart(4, "0")}`, role: "spectator" as const, createdAtMs: 1000 - index, expiresAtMs: 900000, revoked: false, accepted: false });
  const memberRows = (count: number) => Array.from({ length: count }, (_, index) => memberRow(index)).sort((left, right) => (left.userId < right.userId ? -1 : left.userId > right.userId ? 1 : 0));
  const inviteRows = (count: number) => Array.from({ length: count }, (_, index) => inviteRow(index)).sort((left, right) => (right.createdAtMs - left.createdAtMs) || (left.inviteId < right.inviteId ? 1 : -1));

  const seal = (access: "author" | "member" | "public", members: number, invites: number): { canonical: string; page: Record<string, unknown> } => {
    const publicSpace = { id: fixture.space.id, name: fixture.space.name, kind: fixture.space.kind, visibility: fixture.space.visibility, memberCount: fixture.space.memberCount, documentCount: fixture.space.documentCount, createdAtMs: fixture.space.createdAtMs, updatedAtMs: fixture.space.updatedAtMs };
    const memberSpace = { ...fixture.space, role: access === "author" ? "author" : "spectator" };
    const base = {
      access,
      schema: "semio.directory.space-administration-page.v1" as const,
      sessionBindingSha256: access === "public" ? "0".repeat(64) : binding,
      authorizationGeneration: access === "public" ? 0 : fixture.session.authorizationGeneration,
      spaceId: fixture.session.spaceId,
      space: access === "public" ? publicSpace : memberSpace,
    };
    const unsigned = access === "public"
      ? { ...base, documents: { rows: [] } }
      : access === "member"
        ? { ...base, members: { rows: memberRows(members) }, documents: { rows: [] } }
        : { ...base, members: { rows: memberRows(members) }, documents: { rows: [] }, invites: { rows: inviteRows(invites) }, capabilities };
    const receiptSha256 = createHash("sha256").update(JSON.stringify(unsigned)).digest("hex");
    const page = { ...unsigned, receiptSha256 } as Record<string, unknown>;
    return { canonical: JSON.stringify(page), page };
  };

  for (const vector of fixture.vectors) {
    const { canonical, page } = seal(vector.access, vector.expected.memberRows, vector.expected.inviteRows);
    if (("members" in page) !== vector.expected.hasMembers || ("invites" in page) !== vector.expected.hasInvites || ("capabilities" in page) !== vector.expected.hasCapabilities) {
      throw new Error(`space administration shape differs for ${vector.name}`);
    }
    const rows = (page.members as { rows: unknown[] } | undefined)?.rows.length ?? 0;
    const invited = (page.invites as { rows: unknown[] } | undefined)?.rows.length ?? 0;
    if (rows !== vector.expected.memberRows || invited !== vector.expected.inviteRows) throw new Error(`space administration window differs for ${vector.name}`);
    if (rows > fixture.limits.windowRows || invited > fixture.limits.windowRows) throw new Error(`space administration window exceeded ${fixture.limits.windowRows} rows`);
    if (Buffer.byteLength(canonical, "utf8") > fixture.limits.pageBytes) throw new Error(`space administration page exceeded ${fixture.limits.pageBytes} bytes for ${vector.name}`);
    const { receiptSha256, ...unsigned } = page as Record<string, unknown> & { receiptSha256: string };
    if (createHash("sha256").update(JSON.stringify(unsigned)).digest("hex") !== receiptSha256) throw new Error(`space administration receipt differs for ${vector.name}`);
    for (const secret of ["selector", "secretDigest", "inviteToken", "passwordHash", "ssoSubject", "ssoProvider", "sessionId"]) {
      if (canonical.includes(secret)) throw new Error(`space administration page leaked ${secret} in ${vector.name}`);
    }
    if (vector.access !== "author" && (canonical.includes("\"invites\"") || canonical.includes("\"capabilities\""))) throw new Error(`non-author page carried an author-only window in ${vector.name}`);
  }

  const author = seal("author", 2, 2);
  const memberOrder = (page: string): boolean => {
    const parsed = JSON.parse(page) as { members?: { rows: { userId: string }[] } };
    const rows = parsed.members?.rows ?? [];
    return rows.every((row, index) => index === 0 || rows[index - 1]!.userId < row.userId);
  };
  const inviteOrder = (page: string): boolean => {
    const parsed = JSON.parse(page) as { invites?: { rows: { createdAtMs: number; inviteId: string }[] } };
    const rows = parsed.invites?.rows ?? [];
    return rows.every((row, index) => index === 0 || rows[index - 1]!.createdAtMs > row.createdAtMs || (rows[index - 1]!.createdAtMs === row.createdAtMs && rows[index - 1]!.inviteId > row.inviteId));
  };
  if (!memberOrder(author.canonical) || !inviteOrder(author.canonical)) throw new Error("sealed author page is not keyset-ordered");

  const canonicalRejects = (candidate: string): boolean => {
    if (Buffer.byteLength(candidate, "utf8") > fixture.limits.pageBytes) return true;
    let parsed: Record<string, unknown>;
    try { parsed = JSON.parse(candidate) as Record<string, unknown>; } catch { return true; }
    if (JSON.stringify(parsed) !== candidate) return true;
    if (parsed.spaceId !== (parsed.space as { id?: unknown } | undefined)?.id) return true;
    if (!memberOrder(candidate) || !inviteOrder(candidate)) return true;
    const known = new Set(["access", "schema", "sessionBindingSha256", "authorizationGeneration", "spaceId", "space", "members", "documents", "invites", "capabilities", "receiptSha256"]);
    if (Object.keys(parsed).some((key) => !known.has(key))) return true;
    const { receiptSha256, ...unsigned } = parsed as Record<string, unknown> & { receiptSha256: string };
    return createHash("sha256").update(JSON.stringify(unsigned)).digest("hex") !== receiptSha256;
  };
  const hostile = (name: string): string => {
    switch (name) {
      case "receipt-substituted": return author.canonical.replace(/"receiptSha256":"[0-9a-f]{64}"/u, `"receiptSha256":"${"b".repeat(64)}"`);
      case "trailing-whitespace": return `${author.canonical} `;
      case "unknown-field": return author.canonical.replace('{"access":"author"', '{"actor":"user:secret","access":"author"');
      case "space-mismatch": return author.canonical.replace('"spaceId":"space-admin-01"', '"spaceId":"space-admin-02"');
      case "member-order-reversed": return JSON.stringify({ ...JSON.parse(author.canonical), members: { rows: [...memberRows(2)].reverse() } });
      case "invite-order-reversed": return JSON.stringify({ ...JSON.parse(author.canonical), invites: { rows: [...inviteRows(2)].reverse() } });
      case "window-row-max-plus-one": return JSON.stringify({ ...JSON.parse(author.canonical), members: { rows: memberRows(fixture.limits.windowRows + 1) } });
      case "page-byte-max-plus-one": return JSON.stringify({ ...JSON.parse(author.canonical), space: { ...(JSON.parse(author.canonical) as { space: Record<string, unknown> }).space, name: "x".repeat(fixture.limits.pageBytes) } });
      default: return JSON.stringify({ ...JSON.parse(author.canonical), invites: { rows: [{ ...inviteRows(1)[0], secretDigest: "ff".repeat(32) }] } });
    }
  };
  for (const name of fixture.hostiles) {
    const candidate = hostile(name);
    if (name === "window-row-max-plus-one") {
      const rows = (JSON.parse(candidate) as { members: { rows: unknown[] } }).members.rows.length;
      if (rows <= fixture.limits.windowRows) throw new Error("window max+1 hostile did not exceed the ceiling");
      continue;
    }
    if (name === "invite-secret-field" && !candidate.includes("secretDigest")) throw new Error("secret-shaped hostile lost its field");
    if (!canonicalRejects(candidate)) throw new Error(`space administration parser admitted hostile ${name}`);
  }

  const admitCursor = (query: string): string | null => {
    if (query.length === 0) return "";
    if (query.includes("&") || query.includes("%") || query.includes("+")) return null;
    const separator = query.indexOf("=");
    if (separator < 0) return null;
    const name = query.slice(0, separator);
    const value = query.slice(separator + 1);
    if (name !== "cursor" || value.length === 0 || value.length > fixture.limits.cursorBytes || !/^[A-Za-z0-9._-]+$/u.test(value)) return null;
    return value;
  };
  for (const cursorCase of fixture.cursorCases) {
    const admitted = admitCursor(cursorCase.query) !== null;
    const status = admitted ? 200 : 400;
    const reads = admitted ? 1 : 0;
    if (status !== cursorCase.status || reads !== cursorCase.reads) throw new Error(`space administration cursor admission differs for "${cursorCase.query}"`);
  }

  const unknownFixture = { ...fixture, unknown: true };
  if (validate(unknownFixture)) throw new Error("space administration fixture schema admitted an unknown field");

  // 🧬️ Schema-first parity: the SHARED component JSON schema (the one the Rust and TypeScript twins are
  // both derived from) must itself admit every sealed vector and reject every structural hostile. Without
  // this the component `$defs` could drift away from both implementations unnoticed.
  const componentSchema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8")) as { $defs: Record<string, unknown> };
  const validatePage = new Ajv2020({ strict: false, allErrors: true }).compile({ $ref: "#/$defs/DirectorySpaceAdministrationPageV1", ...componentSchema });
  for (const vector of fixture.vectors) {
    const { page } = seal(vector.access, vector.expected.memberRows, vector.expected.inviteRows);
    if (!validatePage(page)) throw new Error(`component schema rejected the sealed ${vector.name} page: ${JSON.stringify(validatePage.errors)}`);
  }
  for (const name of ["unknown-field", "window-row-max-plus-one", "invite-secret-field"]) {
    if (validatePage(JSON.parse(hostile(name)))) throw new Error(`component schema admitted hostile ${name}`);
  }
  const memberShaped = JSON.parse(seal("member", 1, 0).canonical) as Record<string, unknown>;
  if (validatePage({ ...memberShaped, invites: { rows: [] } })) throw new Error("component schema admitted an invite window on a member page");
  if (validatePage({ ...memberShaped, capabilities: { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true } })) throw new Error("component schema admitted capability flags on a member page");

  const contract = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"), "utf8");
  const typescript = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts"), "utf8");
  const hub = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");
  const sqlite = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🪶️sqlite/🦀️.rs"), "utf8");
  const postgres = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🐘️postgres/🦀️.rs"), "utf8");
  const neo4j = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🌐️neo4j/🦀️.rs"), "utf8");
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  const space = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  const sourceClosed = (rust: string, ts: string, route: string, sq: string, pg: string, neo: string, browser: string, home: string): boolean => {
    const read = route.indexOf("list_space_administration_members_page(space_id, member_after.as_deref(), SPACE_ADMINISTRATION_PAGE_FETCH_MAX)");
    const revalidate = route.indexOf("revalidate_space_administration_caller(state, caller.as_ref(), space_id, binding, &space, access)", read);
    return rust.includes("pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS: usize = 64")
      && rust.includes("pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES: usize = 48 * 1024")
      && rust.includes("pub enum DirectorySpaceAdministrationPageV1")
      && !rust.includes("pub enum DirectorySpaceDetailV1")
      && ts.includes("export async function parseDirectorySpaceAdministrationPageV1")
      && ts.includes("space-administration-page.noncanonical")
      && route.includes("fn space_administration_request_admission")
      && route.includes("semio/hub/directory-space-administration/session-binding/v1\\0")
      && route.includes("name != \"cursor\" || value.is_empty() || value.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES")
      && read >= 0 && revalidate > read
      && route.includes("StatusCode::FORBIDDEN")
      && !route.includes("DirectorySpaceDetailV1")
      && sq.includes("SELECT u.id, u.email, u.display_name, m.role")
      && !sq.includes("SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_space_invite WHERE space_id = ?1 AND")
      && pg.includes("async fn list_space_administration_members_page")
      && pg.includes("async fn list_space_administration_invites_page")
      && neo.includes("async fn list_space_administration_members_page")
      && neo.includes("async fn list_space_administration_invites_page")
      && browser.includes("function terminateDirectoryAdministration")
      && browser.includes("const DIRECTORY_ADMINISTRATION_CAPACITY = 1")
      && directoryAdministrationTerminateErases(browser)
      && browser.includes("revokeDirectoryAdministrationForScope(scope.spaceId);")
      && home.includes("row.role == Some(crate::DirectorySpaceRole::Author)");
  };
  if (!sourceClosed(contract, typescript, hub, sqlite, postgres, neo4j, worker, space)) throw new Error("space administration source boundary is incomplete");
  const sourceHostiles: readonly [string, string, string, string, string, string, string, string][] = [
    [contract.replace("pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS: usize = 64", "pub const DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS: usize = 4096"), typescript, hub, sqlite, postgres, neo4j, worker, space],
    [contract, typescript.replace("export async function parseDirectorySpaceAdministrationPageV1", "async function parseDirectorySpaceAdministrationPageV1"), hub, sqlite, postgres, neo4j, worker, space],
    [contract, typescript, hub.replace("revalidate_space_administration_caller(state, caller.as_ref(), space_id, binding, &space, access)", "Ok(access)"), sqlite, postgres, neo4j, worker, space],
    [contract, typescript, hub.replace("name != \"cursor\" || value.is_empty() || value.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES", "false"), sqlite, postgres, neo4j, worker, space],
    [contract, typescript, hub, sqlite, postgres.replace("async fn list_space_administration_invites_page", "async fn unused_invites_page"), neo4j, worker, space],
    [contract, typescript, hub, sqlite, postgres, neo4j.replace("async fn list_space_administration_members_page", "async fn unused_members_page"), worker, space],
    [contract, typescript, hub, sqlite, postgres, neo4j, worker.replace("  operation.inviteToken = null;\n", ""), space],
    [contract, typescript, hub, sqlite, postgres, neo4j, worker.replace("revokeDirectoryAdministrationForScope(scope.spaceId);", ""), space],
    [contract, typescript, hub, sqlite, postgres, neo4j, worker, space.replace("row.role == Some(crate::DirectorySpaceRole::Author)", "true")],
  ];
  sourceHostiles.forEach((candidate, index) => { if (sourceClosed(...candidate)) throw new Error(`space administration source oracle admitted removed fence ${index}`); });
  const checks = fixture.vectors.length * 2 + fixture.cursorCases.length + fixture.hostiles.length + sourceHostiles.length + 7;
  console.log(`space-administration-oracle: AJV=2 vectors=${fixture.vectors.length} cursors=${fixture.cursorCases.length} hostiles=${fixture.hostiles.length} source-hostiles=${sourceHostiles.length} component-schema=${fixture.vectors.length + 5} sha256=1 binding=1`);
  return checks;
}

class SpaceAdministrationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native"].includes(phase)) throw new Error("space-administration-check accepts source or native");
    const checks = await proveDirectorySpaceAdministrationPageV1(this.repoRoot);
    if (phase === "native") {
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [
          {
            package: "semio-hub",
            target: { kind: "bin", name: "os-hub" },
            cargoArgs: [],
            laws: [
              "space_administration_page_v1_route_returns_the_author_windows_with_a_canonical_receipt",
              "space_administration_page_v1_route_denies_a_spectator_the_author_windows",
              "space_administration_page_v1_route_denies_a_removed_member_and_leaks_no_rows",
              "space_administration_page_v1_route_rejects_a_noncanonical_query_and_a_foreign_cursor",
            ],
          },
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) { console.log(`space-administration-${phase} ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`space-administration-${phase}-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`space-administration-check: checks=${checks} phase=${phase}`);
  }
}

async function provePresenceLeaseFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️fixture/🔣️.json"), "utf8")) as PresenceLeaseFixture;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`presence lease fixture: ${JSON.stringify(validate.errors)}`);
  if (new Set(fixture.vectors.map(({ name }) => name)).size !== fixture.vectors.length) throw new Error("presence lease fixture names are not unique");
  type Slot = { liveId: string; deadline: number; peerTag: string | null; peerBytes: number };
  for (const vector of fixture.vectors) {
    const slots = new Map<string, Slot>();
    const outcomes: string[] = [];
    let fanoutCount = 0;
    const roster = (scope: string) => [...slots.entries()]
      .filter(([key, slot]) => key.startsWith(`${scope}\0`) && slot.peerTag !== null)
      .map(([key, slot]) => ({ actor: key.slice(scope.length + 1), bytes: slot.peerBytes }))
      .sort((left, right) => left.actor.localeCompare(right.actor));
    const publish = () => { fanoutCount += 1; outcomes.push("published"); };
    for (const operation of vector.operations) {
      if (operation.kind === "restart") {
        slots.clear();
        outcomes.push("restarted");
        continue;
      }
      if (operation.kind === "fill") {
        for (let index = 0; index < operation.count; index += 1) {
          const suffix = String(index).padStart(3, "0");
          slots.set(`${operation.scope}\0actor-${suffix}`, { liveId: `fill-${suffix}`, deadline: operation.nowMs + fixture.limits.ttlMs, peerTag: `fill-${suffix}`, peerBytes: operation.peerBytes });
        }
        outcomes.push("no-change");
        continue;
      }
      const key = `${operation.scope}\0${operation.actor}`;
      const current = slots.get(key);
      if (operation.kind === "install") {
        slots.set(key, { liveId: operation.liveId, deadline: operation.nowMs + fixture.limits.ttlMs, peerTag: null, peerBytes: 0 });
        if (current?.peerTag !== null && current !== undefined) publish(); else outcomes.push("no-change");
        continue;
      }
      if (!current || current.liveId !== operation.liveId) {
        outcomes.push("no-change");
        continue;
      }
      if (operation.kind === "refresh") {
        const visible = roster(operation.scope);
        const total = visible.reduce((sum, entry) => sum + entry.bytes, 0);
        const wasVisible = current.peerTag !== null;
        if (operation.peerBytes > fixture.limits.maximumEntryBytes || (!wasVisible && visible.length >= fixture.limits.maximumItems) || total - (wasVisible ? current.peerBytes : 0) + operation.peerBytes > fixture.limits.maximumBytes) {
          outcomes.push("rejected");
          continue;
        }
        const changed = current.peerTag !== operation.peerTag || current.peerBytes !== operation.peerBytes;
        current.deadline = operation.nowMs + fixture.limits.ttlMs;
        current.peerTag = operation.peerTag;
        current.peerBytes = operation.peerBytes;
        if (changed) publish(); else outcomes.push("no-change");
      } else if (operation.kind === "tick") {
        if (current.peerTag !== null && operation.nowMs >= current.deadline) {
          current.peerTag = null;
          current.peerBytes = 0;
          publish();
        } else outcomes.push("no-change");
      } else {
        slots.delete(key);
        if (current.peerTag !== null) publish(); else outcomes.push("no-change");
      }
    }
    const final = vector.expected.final.map(expected => {
      const rows = roster(expected.scope);
      return { scope: expected.scope, count: rows.length, bytes: rows.reduce((sum, row) => sum + row.bytes, 0), actors: expected.actors.length === 0 && rows.length > 0 ? [] : rows.map(row => row.actor) };
    });
    const actual = { outcomes, fanoutCount, final, durableWrites: 0 };
    const expected = { outcomes: vector.expected.outcomes, fanoutCount: vector.expected.fanoutCount, final: vector.expected.final, durableWrites: vector.expected.durableWrites };
    if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`presence lease model differs for ${vector.name}: ${JSON.stringify(actual)}`);
  }
  const schemaHostiles: unknown[] = [
    { ...fixture, unknown: true },
    { ...fixture, limits: { ...fixture.limits, ttlMs: 14999 } },
    { ...fixture, vectors: fixture.vectors.map((vector, index) => index === 0 ? { ...vector, operations: [{ ...vector.operations[0], clientDeadlineMs: 1 }] } : vector) },
    { ...fixture, vectors: fixture.vectors.map((vector, index) => index === 0 ? { ...vector, expected: { ...vector.expected, directoryRecipients: ["outsider"] } } : vector) },
  ];
  if (schemaHostiles.some(candidate => validate(candidate))) throw new Error("presence lease schema admitted client authority or an unbounded projection");
  const hub = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");
  const sourceClosed = (source: string): boolean => {
    const publish = source.indexOf("self.fanout_for(key).send(ServerFrame::Presence");
    const directory = source.indexOf("self.directory_service.publish(DirectoryStreamMessage::Presence", publish);
    return source.includes("const PRESENCE_LEASE_TTL_MS: u64 = 15_000")
      && source.includes("socket_live_id: String")
      && source.includes("expires_at: tokio::time::Instant")
      && source.includes("presence_publication_gate: Arc<tokio::sync::Mutex<()>>")
      && source.includes("slot.socket_live_id == socket_live_id")
      && source.includes("remove_if(&map_key, |slot| slot.socket_live_id == socket_live_id)")
      && source.includes("rows.sort_by(|left, right| left.0.cmp(&right.0))")
      && source.includes("peer.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES")
      && source.includes("visible >= PRESENCE_ROSTER_MAXIMUM_ITEMS")
      && source.includes("next > PRESENCE_ROSTER_MAXIMUM_BYTES")
      && source.includes("now >= slot.expires_at")
      && source.includes("state.install_presence_slot(")
      && source.includes("state.refresh_presence(")
      && source.includes("state.expire_presence_for_live(")
      && source.includes("state.close_presence_for_live(")
      && publish >= 0 && directory > publish
      && !source.includes("PresenceSession")
      && !source.includes("connected_at_ms + PRESENCE_LEASE_TTL_MS");
  };
  if (!sourceClosed(hub)) throw new Error("presence lease production ownership/publication fence is incomplete");
  const sourceHostiles = [
    hub.replace("const PRESENCE_LEASE_TTL_MS: u64 = 15_000", "const PRESENCE_LEASE_TTL_MS: u64 = 5_000"),
    hub.replaceAll("slot.socket_live_id == socket_live_id", "true"),
    hub.replace("remove_if(&map_key, |slot| slot.socket_live_id == socket_live_id)", "remove(&map_key)"),
    hub.replace("rows.sort_by(|left, right| left.0.cmp(&right.0));", ""),
    hub.replace("visible >= PRESENCE_ROSTER_MAXIMUM_ITEMS", "false"),
  ];
  sourceHostiles.forEach((source, index) => { if (sourceClosed(source)) throw new Error(`presence lease source oracle admitted removed fence ${index}`); });
  const shell = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"), "utf8");
  const helpers = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx"), "utf8");
  if (!helpers.includes("PRESENCE_HEARTBEAT_INTERVAL_MS = 5000") || !shell.includes("window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS)") || !shell.includes("window.clearInterval(timer)")) throw new Error("browser presence schedule is not a bounded five-second lifecycle");
  const checks = fixture.vectors.length + schemaHostiles.length + sourceHostiles.length + 1;
  console.log(`presence-lease-oracle: AJV=1 vectors=${fixture.vectors.length} schema-hostiles=${schemaHostiles.length} source-hostiles=${sourceHostiles.length} browser-schedule=1`);
  return checks;
}

/** 🪪️ Pins canonical admitted presence bytes independently with AJV and third-party LEB128. */
async function provePresenceNormalizationFixture(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🧪️fixture/🔣️.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8")));
  if (!validate(fixture)) throw new Error(`presence normalization schema: ${JSON.stringify(validate.errors)}`);
  const { default: leb } = await import("@webassemblyjs/leb128/lib/leb.js");
  const integer = (value: number): number[] => {
    if (!Number.isSafeInteger(value) || value < 0) throw new Error("neutral oracle integer outside the exact safe range");
    return Array.from(leb.encodeUInt64(value));
  };
  const text = (value: string): number[] => { const bytes = Buffer.from(value); return [...integer(bytes.length), ...bytes]; };
  const float = (value: number): number[] => { const bytes = Buffer.alloc(8); bytes.writeDoubleLE(value); return Array.from(bytes); };
  const independentEncode = (peer: ArtifactPresencePeer): Buffer => {
    const fields = [peer.label, peer.presencePack, peer.userId, peer.role, peer.dragGhostJson, peer.interaction, peer.color, peer.surface, peer.views.length ? peer.views : undefined, peer.ui];
    const flags = fields.reduce<number>((mask, value, index) => value === undefined ? mask : mask | (1 << index), 0);
    const out = [...text(peer.actor), ...integer(flags), ...integer(peer.connectedAtMs)];
    for (const [index, value] of fields.entries()) {
      if (value === undefined) continue;
      if ([0, 2, 3, 4, 7].includes(index)) out.push(...text(value as string));
      else if (index === 1) { const bytes = value as readonly number[]; out.push(...integer(bytes.length), ...bytes); }
      else if (index === 5) {
        const interaction = peer.interaction!;
        out.push(...text(interaction.app_id), ...integer(interaction.domains.length));
        for (const domain of interaction.domains) {
          out.push(...text(domain.domain), ...text(domain.granularity));
          for (const ids of [domain.selected, domain.hovered]) { out.push(...integer(ids.length)); for (const id of ids) out.push(...text(id)); }
        }
      } else if (index === 6) out.push(value as number);
      else if (index === 8) {
        out.push(...integer(peer.views.length));
        for (const view of peer.views) {
          out.push(...text(view.windowId), ...text(view.space));
          const kind = view.kind;
          const values = kind.kind === "canvas" ? [0, kind.x, kind.y, kind.zoom] : kind.kind === "orbit" ? [1, ...kind.position, ...kind.target, ...kind.up, kind.fov] : [2, kind.lng, kind.lat, kind.zoom, kind.bearing, kind.pitch];
          out.push(values[0]!); for (const number of values.slice(1)) out.push(...float(number));
          out.push(...float(view.size[0]), ...float(view.size[1]), view.pointer ? 1 : 0);
          if (view.pointer) for (const number of view.pointer) out.push(...float(number));
        }
      } else if (index === 9) {
        for (const path of [peer.ui!.hoveredPath, peer.ui!.focusedPath, peer.ui!.pressedPath]) { out.push(path === undefined ? 0 : 1); if (path !== undefined) out.push(...text(path)); }
      }
    }
    return Buffer.from(out);
  };
  for (const vector of fixture.vectors) {
    const raw = Buffer.from(vector.rawPeerHex, "hex");
    let normalized: Buffer | undefined;
    try {
      const input = decodePresencePeer(raw, [0]);
      if (!independentEncode(input).equals(raw)) throw new Error("raw canonical oracle mismatch");
      const admitted = vector.admission;
      const output: ArtifactPresencePeer = {
        actor: admitted.actor, connectedAtMs: admitted.connectedAtMs, label: admitted.label ?? undefined, userId: admitted.userId ?? undefined,
        role: admitted.role ?? undefined, color: admitted.color, surface: admitted.surface ?? undefined,
        presencePack: input.presencePack, dragGhostJson: input.dragGhostJson, interaction: input.interaction, views: input.views, ui: input.ui,
      };
      normalized = independentEncode(output);
      if (!normalized.equals(Buffer.from(encodePresencePeer(output)))) throw new Error("output canonical oracle mismatch");
      decodePresencePeer(normalized, [0]);
    } catch { normalized = undefined; }
    if ((normalized !== undefined) !== vector.expected.accepted || (normalized?.toString("hex") ?? null) !== vector.expected.normalizedPeerHex) throw new Error(`presence normalization vector failed: ${vector.name}`);
  }
  console.log(`presence-normalization-independent-oracle: AJV=1 LEB128=1 exact-vectors=${fixture.vectors.length}`);
  const hub = readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/🚀️bin.rs"), "utf8");
  const start = hub.indexOf("async fn refresh_document_presence(");
  const end = hub.indexOf("async fn refresh_presence(", start);
  const ingress = hub.slice(start, end);
  if (start < 0 || !ingress.includes("protocol::decode_presence_peer(&peer).await") || !ingress.includes("protocol::PresencePeer {") || !ingress.includes("protocol::encode_presence_peer(&normalized).await") || !ingress.includes("self.refresh_presence(")) throw new Error("Hub lacks canonical admitted presence reconstruction");
  for (const field of ["connected_at_ms: slot.connected_at_ms", "label: slot.label.clone()", "user_id: slot.user_id.clone()", "role: slot.role.clone()", "color: Some(slot.color)", "surface: slot.document_surface.clone()"])
    if (!ingress.includes(field)) throw new Error(`Hub presence authority missing: ${field}`);
  if (!hub.includes("state.refresh_document_presence(") || !hub.includes("socket_grant.document_plan.as_ref().map(|plan| plan.surface.surface_id.clone())")) throw new Error("presence ingress must use the admitted plan surface");
  return fixture.vectors.length;
}

class PresenceNormalizationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native"].includes(phase)) throw new Error("presence-normalization-check accepts source or native");
    const checks = await provePresenceNormalizationFixture(this.repoRoot);
    if (phase === "native") {
      const receipts = await runExactCargoLaws({
        cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-hub", target: { kind: "bin", name: "os-hub" }, cargoArgs: ["--features", "sqlite"], laws: [
          "presence_normalization_matches_neutral_authority_and_no_effect_rejections",
          "presence_normalization_socket_overwrites_identity_and_rejects_without_refresh",
          "presence_lease_reconnect_rejects_old_live_refresh_and_close",
          "presence_lease_expires_server_clocked_visibility_without_socket_close",
          "presence_lease_enforces_shared_roster_bounds_and_actor_order",
          "presence_lease_restart_is_empty_and_directory_presence_is_member_only",
        ] }],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: buildBudgetMs(), listBudgetMs: 60_000, lawBudgetMs: 60_000,
        progress(event) { console.log(`presence-normalization-native ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`presence-normalization-native-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`presence-normalization-check: checks=${checks} phase=${phase}`);
  }
}

/** 🛂️ Qualifies durable membership removal across live presence and selected target admission. */
class AdminPresenceTargetRecoveryCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native"].includes(phase)) throw new Error("admin-presence-target-recovery-check accepts source or native");
    const root = join(this.repoRoot, "🌎️hub/📦️packages/🦀️rust");
    const fixtureRoot = join(root, "🧪️fixtures/🛂️admin-presence-target-recovery-v1");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8")));
    if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));
    const members = fixture.members.map((member: { id: string }) => member.id).filter((id: string) => id !== fixture.remove);
    if (JSON.stringify(members) !== JSON.stringify(fixture.expected.members) || new Set(fixture.members.map((member: { id: string }) => member.id)).size !== 2) throw new Error("membership recovery oracle differs");
    console.log("admin-presence-target-recovery-independent-oracle: AJV=1 scoped-removal=1");
    const law = "admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen";
    const hub = readFileSync(join(root, "🚀️bin.rs"), "utf8");
    if (!hub.includes(`fn ${law}(`) || !hub.includes("with_graceful_shutdown") || !hub.includes("test_state_with_directory")) throw new Error("missing composed SQLite shutdown/reopen acceptance journey");
    if (hub.match(/const STUDIO: &str = "([^"]+)";/)?.[1] !== fixture.scope.spaceId) throw new Error("recovery fixture differs from seeded test space");
    if (phase === "native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot, env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{ package: "semio-hub", target: { kind: "bin", name: "os-hub" }, cargoArgs: ["--features", "sqlite"], laws: [law] }],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: buildBudgetMs(), listBudgetMs: 60_000, lawBudgetMs: 120_000,
        progress(event) { console.log(`admin-presence-target-recovery-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`admin-presence-target-recovery-native-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`admin-presence-target-recovery-check: phase=${phase}`);
  }
}

class PresenceLeaseCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native", "process"].includes(phase)) throw new Error("presence-lease-check accepts source, native, or process");
    const checks = await provePresenceLeaseFixture(this.repoRoot);
    if (phase === "native" || phase === "process") {
      const hubLaws = [
        "presence_lease_reconnect_rejects_old_live_refresh_and_close",
        "presence_lease_expires_server_clocked_visibility_without_socket_close",
        "presence_lease_enforces_shared_roster_bounds_and_actor_order",
        "presence_lease_restart_is_empty_and_directory_presence_is_member_only",
      ];
      const hubGroup = { package: "semio-hub", target: { kind: "bin" as const, name: "os-hub" }, cargoArgs: ["--all-features"], laws: hubLaws };
      const groups = phase === "native"
        ? [
            { package: "semio-framework-os-kernel", target: { kind: "lib" as const, name: "semio_framework_os_kernel" }, laws: ["presence_roster_fixed_maximum_plus_one_returns_the_exact_rejected_owner"] },
            hubGroup,
          ]
        : [hubGroup];
      const receipts = await runExactCargoLaws({
        cwd: phase === "native" ? this.repoRoot : this.root,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups,
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: buildBudgetMs(),
        listBudgetMs: 60_000,
        lawBudgetMs: 60_000,
        progress(event) { console.log(`presence-lease-${phase} ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      for (const receipt of receipts) console.log(`presence-lease-${phase}-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`presence-lease-check: checks=${checks} phase=${phase}`);
  }
}

/** 🎟️ Evaluates the neutral one-transaction invite state machine independently of every backend. */
async function proveInviteRedemptionTransaction(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as InviteRedemptionFixture;
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`invite redemption transaction fixture: ${JSON.stringify(validate.errors)}`);
  if (new Set(fixture.vectors.map(({ name }) => name)).size !== fixture.vectors.length) throw new Error("invite redemption fixture names are not unique");
  for (const vector of fixture.vectors) {
    const accepted = vector.initial === "accepted-same" || vector.initial === "accepted-other" || vector.initial === "corrupt-marker" || vector.initial === "corrupt-event";
    let acceptedAt: number | null = accepted ? 100 : null;
    let acceptedEventId: string | null = vector.initial === "corrupt-marker" ? null : vector.initial === "corrupt-event" ? "event-missing" : accepted ? "event-0" : null;
    let event = vector.initial === "accepted-same" ? { id: "event-0", user: "same", recordedAt: 100 } : vector.initial === "accepted-other" ? { id: "event-0", user: "other", recordedAt: 100 } : null;
    let membership = event ? 1 : 0;
    let publications = 0;
    let revoked = vector.initial === "revoked";
    const recordExists = vector.initial !== "missing";
    const userExists = vector.initial !== "missing-user";
    const spaceExists = vector.initial !== "missing-space";
    const outcomes: string[] = [];
    const returnedEventIds: (string | null)[] = [];
    for (const call of vector.calls) {
      if (call.kind === "restart") {
        outcomes.push("restarted");
        returnedEventIds.push(null);
        continue;
      }
      if (call.kind === "rebuild") {
        membership = event ? 1 : 0;
        outcomes.push("rebuilt");
        returnedEventIds.push(null);
        continue;
      }
      if (call.kind === "append-forged") {
        outcomes.push("append-denied");
        returnedEventIds.push(null);
        continue;
      }
      if (call.kind === "revoke") {
        if (acceptedAt !== null || revoked) outcomes.push("conflict");
        else {
          revoked = true;
          outcomes.push("revoked");
        }
        returnedEventIds.push(null);
        continue;
      }
      if (!recordExists || call.actor !== "exact" || call.credential !== "exact") {
        outcomes.push("unauthorized");
        returnedEventIds.push(null);
        continue;
      }
      if ((acceptedAt === null) !== (acceptedEventId === null)) {
        outcomes.push("backend-error");
        returnedEventIds.push(null);
        continue;
      }
      if (acceptedAt !== null) {
        if (!event || event.id !== acceptedEventId || event.recordedAt !== acceptedAt) {
          outcomes.push("backend-error");
          returnedEventIds.push(null);
        } else if (event.user !== call.user) {
          outcomes.push("conflict");
          returnedEventIds.push(null);
        } else {
          outcomes.push("already-committed");
          returnedEventIds.push(event.id);
        }
        continue;
      }
      if (!userExists || !spaceExists || call.user === "missing") {
        outcomes.push("unauthorized");
        returnedEventIds.push(null);
        continue;
      }
      if (revoked || vector.initial === "expired") {
        outcomes.push("conflict");
        returnedEventIds.push(null);
        continue;
      }
      if (call.failure !== "none") {
        outcomes.push("backend-error");
        returnedEventIds.push(null);
        continue;
      }
      acceptedAt = 101;
      acceptedEventId = "event-1";
      event = { id: "event-1", user: call.user, recordedAt: 101 };
      membership = 1;
      publications += 1;
      outcomes.push("newly-committed");
      returnedEventIds.push(event.id);
    }
    const actual = { outcomes, returnedEventIds, marker: { acceptedAt, acceptedEventId }, events: event ? 1 : 0, memberships: membership, publications, replayEvents: event ? 1 : 0, revoked };
    if (JSON.stringify(actual) !== JSON.stringify(vector.expected)) throw new Error(`invite redemption independent model differs for ${vector.name}: ${JSON.stringify(actual)}`);
  }
  for (const hostile of fixture.hostiles) {
    const candidate = structuredClone(fixture) as InviteRedemptionFixture & Record<string, unknown>;
    const call = candidate.vectors[0]!.calls[0]! as unknown as Record<string, unknown>;
    if (hostile.mutation === "raw-capability") call.rawCapability = "forbidden";
    else if (hostile.mutation === "client-space") call.spaceId = "client-space";
    else if (hostile.mutation === "client-role") call.role = "author";
    else if (hostile.mutation === "client-event-id") call.eventId = "client-event";
    else if (hostile.mutation === "unknown-field") candidate.unknown = true;
    else (candidate.vectors[0]!.expected.returnedEventIds as (string | null)[])[0] = "x".repeat(4097);
    if (validate(candidate)) throw new Error(`invite redemption schema admitted hostile ${hostile.name}`);
  }
  const directory = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🦀️.rs"), "utf8");
  const sqlite = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🪶️sqlite/🦀️.rs"), "utf8");
  const postgres = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🐘️postgres/🦀️.rs"), "utf8");
  const neo4j = readFileSync(join(repoRoot, "🌎️hub/📇️directory/🌐️neo4j/🦀️.rs"), "utf8");
  const body = (source: string, signature: string): string => {
    const offset = source.indexOf(signature);
    const start = source.indexOf("{", offset);
    let depth = 0;
    for (let index = start; offset >= 0 && index < source.length; index += 1) {
      if (source[index] === "{") depth += 1;
      else if (source[index] === "}" && --depth === 0) return source.slice(start + 1, index);
    }
    return "";
  };
  const ordered = (source: string, values: readonly string[]): boolean => values.every((value, index) => source.indexOf(value) >= 0 && (index === 0 || source.indexOf(values[index - 1]!) < source.indexOf(value)));
  const sourceClosed = (shared: string, sq: string, pg: string, neo: string): boolean => {
    const service = body(shared, "pub async fn redeem_invite(");
    const sqliteClaim = body(sq, "async fn redeem_invite_atomic(");
    const postgresClaim = body(pg, "async fn redeem_invite_atomic(");
    const neoClaim = body(neo, "async fn redeem_invite_atomic(");
    return shared.includes("pub accepted_event_id: Option<String>")
      && shared.includes("InviteRedemptionCommit::AlreadyCommitted")
      && ordered(service, ["let mut clock = self.write.lock().await", "let hlc = clock.tick()", "self.dir.redeem_invite_atomic", "InviteRedemptionCommit::NewlyCommitted"])
      && service.includes("InviteRedemptionCommit::AlreadyCommitted { event } => Ok(vec![event])")
      && !service.slice(service.indexOf("InviteRedemptionCommit::AlreadyCommitted")).includes("publish_persisted_locked")
      && sq.includes("CHECK ((accepted_at IS NULL) = (accepted_event_id IS NULL))")
      && sqliteClaim.includes("TransactionBehavior::Immediate")
      && ordered(sqliteClaim.slice(sqliteClaim.indexOf("SET accepted_at = ?2")), ["SET accepted_at = ?2, accepted_event_id = ?3", "persist_event_with_identity", "self.project(&tx", "tx.commit()"])
      && pg.includes("CHECK ((accepted_at IS NULL) = (accepted_event_id IS NULL))")
      && postgresClaim.includes("WHERE singleton FOR UPDATE")
      && postgresClaim.includes("WHERE selector = $1 FOR UPDATE")
      && ordered(postgresClaim.slice(postgresClaim.indexOf("SET accepted_at = $2")), ["SET accepted_at = $2, accepted_event_id = $3", "INSERT INTO hub_directory_event", "self.project(&mut tx", "tx.commit()"])
      && pg.includes("CREATE TEMP TABLE hub_rebuild_space_invite ON COMMIT DROP")
      && pg.includes("accepted_at, accepted_event_id FROM hub_rebuild_space_invite")
      && neoClaim.includes("SET c.claimNonce")
      && neoClaim.includes("SET i.claimNonce")
      && ordered(neoClaim.slice(neoClaim.indexOf("SET i.acceptedAt = $accepted_at")), ["SET i.acceptedAt = $accepted_at, i.acceptedEventId = $event_id", "CREATE (e:DirectoryEvent", "self.project(&mut txn", "txn.commit()"])
      && [sq, pg, neo].every((source) => source.includes("DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. }"))
      && [sq, pg, neo].every((source) => source.includes("accepted_at") && source.includes("accepted_event_id"));
  };
  if (!sourceClosed(directory, sqlite, postgres, neo4j)) throw new Error("invite redemption production transaction fence is incomplete");
  const sourceHostiles = [
    [directory.replace("pub accepted_event_id: Option<String>", ""), sqlite, postgres, neo4j],
    [directory.replace("InviteRedemptionCommit::AlreadyCommitted { event } => Ok(vec![event])", "InviteRedemptionCommit::AlreadyCommitted { event } => Ok(self.publish_persisted_locked(&clock, vec![event]))"), sqlite, postgres, neo4j],
    [directory, sqlite.replace("transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;\n        let accepted_at_ms", "transaction_with_behavior(rusqlite::TransactionBehavior::Deferred).map_err(backend)?;\n        let accepted_at_ms"), postgres, neo4j],
    [directory, sqlite, postgres.replaceAll("FOR UPDATE", ""), neo4j],
    [directory, sqlite, postgres.replace("CREATE TEMP TABLE hub_rebuild_space_invite ON COMMIT DROP", ""), neo4j],
    [directory, sqlite, postgres, neo4j.replaceAll("claimNonce", "claimRead")],
  ];
  sourceHostiles.forEach((hostile, index) => {
    if (sourceClosed(hostile[0]!, hostile[1]!, hostile[2]!, hostile[3]!)) throw new Error(`invite redemption source oracle admitted removed transaction fence ${index}`);
  });
  const checks = fixture.vectors.length + fixture.hostiles.length + sourceHostiles.length;
  console.log(`invite-redemption-transaction-oracle: AJV=1 vectors=${fixture.vectors.length} hostiles=${fixture.hostiles.length} source-hostiles=${sourceHostiles.length}`);
  return checks;
}

class InviteRedemptionTransactionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && !["--native", "--postgres", "--neo4j"].includes(segments[0]!))) throw new Error("invite-redemption-transaction-check accepts only --native, --postgres, or --neo4j");
    const checks = await proveInviteRedemptionTransaction(this.repoRoot);
    const lawGroups = segments[0] === "--postgres"
      ? [{ package: "semio-hub", target: { kind: "lib" as const }, cargoArgs: ["--features", "postgres"], laws: ["directory::postgres::tests::invite_redemption_claim_matches_neutral_contract"] }]
      : segments[0] === "--neo4j"
        ? [{ package: "semio-hub", target: { kind: "lib" as const }, cargoArgs: ["--features", "neo4j"], laws: ["directory::neo4j::tests::invite_redemption_claim_matches_neutral_contract"] }]
        : segments[0] === "--native"
          ? [{ package: "semio-hub", target: { kind: "lib" as const }, cargoArgs: ["--features", "sqlite"], laws: [
              "directory::tests::invite_redemption_sqlite_claim_is_exactly_once_across_concurrency_restart_and_rebuild",
              "directory::tests::invite_redemption_projection_failure_rolls_back_claim_event_and_membership",
              "directory::tests::invite_redemption_commit_and_publication_precede_the_next_directory_command",
            ] }]
          : [];
    if (lawGroups.length > 0) {
      const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: lawGroups, progress(event) { console.log(`invite-redemption-transaction ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
      console.log(`invite-redemption-transaction-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`invite-redemption-transaction-check: checks=${checks} mode=${segments[0] ?? "source"}`);
  }
}

class DirectoryOrderedPublicationCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-ordered-publication-check accepts only --native");
    const checks = orderedDirectoryPublicationOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.repoRoot,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{
          package: "semio-hub",
          target: { kind: "lib" },
          laws: ["directory::tests::directory_append_and_live_broadcast_share_one_writer_guard_and_projection_order"],
        }],
        progress(event) { console.log(`directory-ordered-publication ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      console.log(`directory-ordered-publication-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-ordered-publication-check: checks=${checks} clean`);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("setup", SetupScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("artifact-cas-check", ArtifactCasCheckScript)
  .register("socket-grant-check", SocketGrantCheckScript)
  .register("scoped-directory-socket-check", ScopedDirectorySocketCheckScript)
  .register("browser-broker-check", BrowserBrokerCheckScript)
  .register("execution-target-relay-check", ExecutionTargetRelayCheckScript)
  .register("admin-relay-check", AdminRelayCheckScript)
  .register("admin-backend-check", AdminBackendCheckScript)
  .register("admin-live-journey-check", AdminLiveJourneyCheckScript)
  .register("invite-redemption-transaction-check", InviteRedemptionTransactionCheckScript)
  .register("presence-lease-check", PresenceLeaseCheckScript)
  .register("presence-normalization-check", PresenceNormalizationCheckScript)
  .register("admin-presence-target-recovery-check", AdminPresenceTargetRecoveryCheckScript)
  .register("directory-event-page-v1-check", DirectoryEventPageV1CheckScript)
  .register("space-administration-check", SpaceAdministrationCheckScript)
  .register("directory-command-receipt-check", DirectoryCommandReceiptCheckScript)
  .register("directory-home-browser-process-check", DirectoryHomeBrowserProcessCheckScript)
  .register("scoped-presence-browser-serve", ScopedPresenceBrowserServeScript)
  .register("directory-ordered-publication-check", DirectoryOrderedPublicationCheckScript)
  .register("canonical-pair-check", CanonicalPairCheckScript)
  .register("native-openable-catalog-provider-check", NativeOpenableCatalogProviderCheckScript)
  .register("native-catalog-selection-check", NativeCatalogSelectionCheckScript)
  .register("open-plan-check", OpenPlanCheckScript)
  .register("open-plan-server-check", OpenPlanServerCheckScript)
  .register("browser-document-open-check", BrowserDocumentOpenCheckScript)
  .register("execution-target-lease-check", ExecutionTargetLeaseCheckScript)
  .register("execution-target-lease-browser-check", ExecutionTargetLeaseBrowserCheckScript)
  .register("space-public-boundary-check", SpacePublicBoundaryCheckScript)
  .register("trusted-stdio-gis-bootstrap", TrustedStdioGisBootstrapScript)
  .register("gis-map-proposal-check", GisMapProposalCheckScript)
  .register("trusted-stdio-gis-bundle-check", TrustedStdioGisBundleCheckScript)
  .register("gis-inference-ledger-oracle", GisInferenceLedgerOracleScript)
  .register("gis-inference-ledger-check", GisInferenceLedgerCheckScript)
  .register("gis-map-frozen-binding-check", GisMapFrozenBindingCheckScript)
  .register("native-document-open-check", NativeDocumentOpenCheckScript)
  .register("dev", DevScript)
  .register("secure-local-smoke", SecureLocalSmokeScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
