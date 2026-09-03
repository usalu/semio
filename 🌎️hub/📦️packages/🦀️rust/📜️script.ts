#!/usr/bin/env bun
import { createHash, createHmac, randomBytes, timingSafeEqual } from "node:crypto";
import { chmodSync, existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { spawn, type ChildProcess } from "node:child_process";
import type { Duplex } from "node:stream";
import { decodeClientFrame, encodeServerFrame, type WireFrontierSummary } from "../../../🧰️framework/🔨️modules/📡️replication/🟦️.ts";
import { decodeBackboneWorkerResponse, encodeBackboneWorkerRequest } from "../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { parseDocumentOpenIntentV1, parseDocumentOpenPlanV1, parseDocumentPlanSocketGrantIntentV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
/** 🌎️ `os-hub` router: `bun ./📜️script.ts <setup|build|test|dev>`. */
import {
  BundleScript,
  ScriptRouter,
  OS_HUB_PORT,
  OS_HUB_PORT_ENV,
  runBundleScriptMain,
  runCargo,
  runCargoTestBudgeted,
  runCmd,
  runProbe,
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

const LOCAL_RELAY_MAX_BODY_BYTES = 1024 * 1024;
const LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES = 4 * 1024 * 1024;
const LOCAL_RELAY_MAX_IN_FLIGHT = 64;
const LOCAL_RELAY_DEADLINE_MS = 2_000;
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

function localRelayUpstreamPath(method: string, url: URL): string | undefined {
  if (!url.pathname.startsWith("/_semio/hub/")) return undefined;
  const upstream = url.pathname.slice("/_semio/hub".length);
  const noQuery = url.search === "";
  if (method === "GET" && upstream === "/auth/sessions/me" && noQuery) return upstream;
  if (method === "GET" && (upstream === "/directory/spaces" || /^\/directory\/spaces\/[^/]+$/u.test(upstream)) && noQuery) return upstream;
  if (method === "GET" && upstream === "/directory/events" && [...url.searchParams].length === 1 && /^\d+$/u.test(url.searchParams.get("since") ?? "")) return `${upstream}?since=${url.searchParams.get("since")}`;
  if (method === "POST" && (upstream === "/directory/commands" || upstream === "/directory/socket-grants") && noQuery) return upstream;
  if (method === "POST" && /^\/spaces\/[^/]+\/documents\/[^/]+\/open-plan$/u.test(upstream) && noQuery) return upstream;
  if (method === "POST" && /^\/spaces\/[^/]+\/documents\/[^/]+\/socket-grants$/u.test(upstream) && noQuery) return upstream;
  return undefined;
}

async function readLocalRelayBody(request: Request, maximumBytes = LOCAL_RELAY_MAX_BODY_BYTES): Promise<Uint8Array | undefined> {
  if (request.method === "GET" || request.method === "DELETE" || request.body === null) return undefined;
  const contentLength = Number(request.headers.get("content-length") ?? "0");
  if (!Number.isSafeInteger(contentLength) || contentLength < 0 || contentLength > maximumBytes) throw new Error("payload too large");
  const reader = request.body.getReader();
  const chunks: Uint8Array[] = [];
  let retained = 0;
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    retained += value.byteLength;
    if (retained > maximumBytes) {
      await reader.cancel();
      throw new Error("payload too large");
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
      const contentLength = Number(request.headers.get("content-length") ?? "0");
      if (!Number.isSafeInteger(contentLength) || contentLength < 0 || contentLength > LOCAL_RELAY_MAX_BODY_BYTES) return new Response("payload too large", { status: 413 });
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
        if (upstream.status === 401) capability = "";
        const responseBody = await readLocalRelayResponse(upstream);
        const contentType = upstream.headers.get("content-type");
        return new Response(responseBody, { status: upstream.status, headers: { "x-semio-browser-broker-advanced": "1", ...(contentType ? { "content-type": contentType } : {}) } });
      } catch (error) {
        return new Response(error instanceof Error && error.message === "payload too large" ? "payload too large" : "unavailable", {
          status: error instanceof Error && error.message === "payload too large" ? 413 : 503,
          headers: { "x-semio-browser-broker-advanced": "1" },
        });
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
  options: { readonly port?: number; readonly dataDir?: string; readonly capture?: boolean; readonly adminSubjects?: readonly string[]; readonly isolatedSecuritySmoke?: boolean } = {},
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
  const outputMode: "pipe" | "inherit" = options.capture ? "pipe" : "inherit";
  const child = spawn(hubBinaryPath(repoRoot), [], { cwd: root, env, shell: false, stdio: ["ignore", outputMode, outputMode, "pipe"] });
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
  const entrypoint = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs"), "utf8");
  const workspace = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"), "utf8");
  const remote = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs"), "utf8");
  const directory = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  const runnerPaths = ["🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️📦️packages/🦀️rust/📜️script.ts"].map((path) => join(repoRoot, path)).filter(existsSync);
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
  if (!directory.includes('"/directory/socket-grants"') || !directory.includes('"/directory/socket/v1"') || !directory.includes("directory_socket_hello_v1()")) throw new Error("MCP directory binding no longer uses the v1 receipt/tag7 protocol");
  if (runner.includes('runCmd("cargo", ["run"') || !runner.includes("runCmd(buildMcpBinary")) throw new Error("MCP runner is not a direct binary supervisor");
  if (!launch.includes("os-hub:dev-secure-mcp")) throw new Error("MCP secure direct-child launch is not registered in the source seed");
}

const MCP_PROBE_SCHEMA = "os.agent.probe/v1";
const MCP_PROBE_PACK_SCHEMA_HASH = "9fab7cb8b71dabede955b4257fa06e2908642e0904f124b6230479f8a153041e";

async function createMcpProbeWorkspace(run: LocalHubRun, envelope: Record<string, any>): Promise<{ readonly spaceId: string; readonly documentId: string }> {
  const response = await fetch(`http://127.0.0.1:${run.port}/directory/commands`, {
    method: "POST",
    headers: { authorization: `Bearer ${envelope.capability}`, "content-type": "application/json" },
    body: JSON.stringify({ kind: "create-space", name: "MCP Socket Grant Probe", spaceKind: "studio", visibility: "private" }),
    signal: AbortSignal.timeout(2_000),
  });
  if (!response.ok) throw new Error(`MCP process probe could not create its workspace: ${response.status}`);
  const body = (await response.json()) as Record<string, any>;
  const event = Array.isArray(body.events) ? body.events.find((candidate: any) => candidate?.body?.kind === "space.created") : undefined;
  const spaceId = event?.body?.spaceId;
  if (typeof spaceId !== "string" || spaceId.length === 0) throw new Error("MCP process probe create-space response lacked its exact identifier");
  const documentId = "mcp-socket-grant-probe";
  const announced = await fetch(`http://127.0.0.1:${run.port}/directory/commands`, {
    method: "POST",
    headers: { authorization: `Bearer ${envelope.capability}`, "content-type": "application/json" },
    body: JSON.stringify({
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
    }),
    signal: AbortSignal.timeout(2_000),
  });
  if (!announced.ok) throw new Error(`MCP process probe could not announce its document: ${announced.status}`);
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
  const entrypoint = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs"), "utf8");
  const credential = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  const runner = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts"), "utf8");
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
  readonly installedTarget: { readonly package: Record<string, any>; readonly artifact: Record<string, any>; readonly surface: Record<string, any> };
  readonly plan: Record<string, any>;
  readonly socketGrant: Record<string, any>;
  readonly expected: { readonly httpPaths: readonly [string, string]; readonly webSocketPath: string; readonly protocol: string; readonly helloSchema: string; readonly helloPackSchemaHashByte: number; readonly responseMaxBytes: number; readonly rustWorkerBypassDenied: true; readonly scopeIsolation: { readonly left: { readonly spaceId: string; readonly documentId: string }; readonly right: { readonly spaceId: string; readonly documentId: string }; readonly leftKey: string; readonly rightKey: string; readonly localKey: string }; readonly forbiddenSocketFragments: readonly string[] };
  readonly hostile: readonly { readonly name: string; readonly stage: string; readonly replacePath?: string; readonly value?: unknown; readonly expected: string }[];
};

async function browserDocumentOpenFixture(repoRoot: string): Promise<BrowserDocumentOpenFixture> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory");
  const schema = JSON.parse(readFileSync(join(root, "📄️browser-document-open-v1.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(root, "📄️browser-document-open-v1.json"), "utf8")) as BrowserDocumentOpenFixture;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`browser document-open fixture invalid: ${JSON.stringify(validate.errors)}`);
  return fixture;
}

function browserDocumentOpenAuthority(plan: Record<string, any>, fixture: BrowserDocumentOpenFixture): boolean {
  const intent = fixture.intent;
  const installed = fixture.installedTarget;
  return plan.scope?.spaceId === intent.scope?.spaceId
    && plan.scope?.documentId === intent.scope?.documentId
    && plan.package?.pluginId === installed.package.pluginId
    && plan.package?.packageId === installed.package.packageId
    && plan.package?.version === installed.package.version
    && plan.package?.componentSha256 === installed.package.componentSha256
    && plan.package?.componentBlake3 === installed.package.componentBlake3
    && plan.package?.descriptorByteSha256 === installed.package.descriptorByteSha256
    && plan.artifact?.kind === installed.artifact.kind
    && plan.artifact?.schema === installed.artifact.schema
    && plan.artifact?.schema === fixture.expected.helloSchema
    && plan.artifact?.packSchemaHash === installed.artifact.packSchemaHash
    && plan.artifact?.packSchemaHash === fixture.expected.helloPackSchemaHashByte.toString(16).padStart(2, "0").repeat(32)
    && plan.surface?.surfaceId === installed.surface.surfaceId
    && plan.surface?.surfaceId === intent.requestedSurfaceId
    && plan.surface?.appId === installed.surface.appId
    && plan.surface?.windowKindId === installed.surface.windowKindId
    && plan.surface?.role === installed.surface.role
    && plan.surface?.rendererTarget === installed.surface.rendererTarget
    && plan.surface?.rendererTarget === "react"
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
    bindings: [{ kind: "hub", baseUrl: hubOrigin, spaceId: current.intent.scope.spaceId, installedTarget: current.installedTarget }],
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
      configFile: join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts"),
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
    }, { workerUrl: `/@fs${join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts")}`, proof: proofHex, openWire });
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
  const fixtureRoot = join(repoRoot, "🌎️hub/📇️directory/🧪️fixtures/🧬️admin-live-journey-v1");
  const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as AdminLiveJourneyFixture;
  const Ajv2020 = (await import("ajv/dist/2020.js")).default;
  const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`admin live journey fixture invalid: ${JSON.stringify(validate.errors)}`);
  const admissionRoot = join(repoRoot, "🌎️hub/🔐️local-bootstrap/🧪️fixtures/🧬️idle-admission-v1");
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

class SocketGrantCheckScript extends BundleScript {
  run(): void {
    const tests = [
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
  const root = documentOpenNeutralObject(candidate, ["schema", "version", "receipt", "expiresAtUnixMs", "scope", "descriptorDigestV1", "catalog", "package", "artifact", "surface", "grant", "revalidation"], ["checkpoint"]);
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
  const fixturePath = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️document-open-plan-v1.json");
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Record<string, any>;
  const hubSource = readFileSync(resolve(repoRoot, "🌎️hub/📦️packages/🦀️rust/📦️bin.rs"), "utf8");
  const routePaths = [...hubSource.matchAll(/\.route\(\s*"([^"]+)"/g)].map((match) => match[1]!);
  const productionSource = hubSource.slice(0, hubSource.indexOf("\nmod tests {"));
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
  async run(): Promise<void> {
    await proveDocumentOpenPlanFixture(this.repoRoot);
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

class BrowserDocumentOpenCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = await proveBrowserDocumentOpenFixture(this.repoRoot);
    await proveBrowserDocumentOpenRuntime(this.repoRoot, fixture);
    runCmd("bun", ["nx", "run", "@semio-tech/framework-os:test-quick", "--skip-nx-cache", "--", "--run", "-t", "browser document open"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCmd("bun", ["./📜️script.ts", "open-plan-server-check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("browser-document-open-check: neutral oracle, browser Worker D1 issue/exchange/WebSocket runtime, Session-gated activation, hostile bounds/cancellation/redaction, and current server laws passed");
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
  console.log(`native-document-admission-laws: exact=${suffixes.length} passed`);
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
    const run = await startLocalHub(this.repoRoot, this.root, profiles, {
      port: Number(process.env[OS_HUB_PORT_ENV] ?? OS_HUB_PORT),
      dataDir: process.env.OS_HUB_DATA,
      adminSubjects: secureAdmin ? ["semio.local.bootstrap/v1:local-administrator-01"] : undefined,
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
        const uiScript = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts");
        ui = spawn(process.execPath, [uiScript, "dev", "s"], {
          cwd: join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript"),
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
    runCmd("bun", ["nx", "run", "os-hub-admin:test", "--skip-nx-cache", "--", "long", "--run", "🧪️admin.test.tsx"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    console.log("admin-relay-check: one-use fragment bootstrap, host-only HttpOnly strict cookie, expiry/replay/CSRF/raw-local denial, bearer redaction, and EN/DE UI laws passed");
  }
}

class AdminBackendCheckScript extends BundleScript {
  async run(): Promise<void> {
    runCmd("bun", [join(this.repoRoot, "🌎️hub/📇️directory/🧪️fixtures/🧬️admin-intent-v1/🧪️oracle/🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
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
    runCmd("bun", ["nx", "run", "os-hub-admin:test", "--skip-nx-cache", "--", "long", "--run", "🧪️admin.test.tsx"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
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

const router = new ScriptRouter(import.meta.dir)
  .register("setup", SetupScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("artifact-cas-check", ArtifactCasCheckScript)
  .register("socket-grant-check", SocketGrantCheckScript)
  .register("browser-broker-check", BrowserBrokerCheckScript)
  .register("admin-relay-check", AdminRelayCheckScript)
  .register("admin-backend-check", AdminBackendCheckScript)
  .register("admin-live-journey-check", AdminLiveJourneyCheckScript)
  .register("canonical-pair-check", CanonicalPairCheckScript)
  .register("open-plan-check", OpenPlanCheckScript)
  .register("open-plan-server-check", OpenPlanServerCheckScript)
  .register("browser-document-open-check", BrowserDocumentOpenCheckScript)
  .register("dev", DevScript)
  .register("secure-local-smoke", SecureLocalSmokeScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
