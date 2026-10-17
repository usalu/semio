/** 🧭️ The local browser and admin relay servers the live hub gates drive.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { GIS_MAP_INFERENCE_REQUEST_MAX_BYTES, GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES } from "@semio-tech/gis-gismap-js/inference/schema";
import { createHash, randomBytes, timingSafeEqual } from "node:crypto";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { DOCUMENT_BROWSER_ACTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema/browser-actor";
import { SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES, SPACE_ARTIFACT_CREATION_MAX_BYTES } from "@semio-tech/framework-os/modules/directory/schema/space-artifact-creation-v1";
import { localRelayExecutionTargetAsset, localRelayInferencePath, localRelaySpaceArtifactCreationPath, localRelayUpstreamPath } from "../🧭️routing/🟦️.ts";

export type AdminLiveJourneyFixture = {
  readonly schema: "semio.hub.admin-live-journey/v1";
  readonly profile: { readonly profileId: string; readonly subject: string; readonly displayName: string };
  readonly limits: { readonly journeyMs: number; readonly pollMs: number; readonly responseBytes: 65536 };
  readonly languages: readonly { readonly locale: "en" | "de"; readonly overview: string; readonly spaces: string; readonly newSpace: string }[];
  readonly mutation: { readonly kind: "create-space"; readonly requestId: string; readonly name: string; readonly spaceKind: "atelier" | "studio" | "archive"; readonly visibility: "private" | "public" };
  readonly operation: { readonly kind: "rebuild-directory-projections"; readonly requestId: string };
};

export type LocalBrowserRelay = { readonly url: string; readonly secret: Buffer; takeBrowserBootstrapProof: () => Buffer; stop: () => Promise<void> };
type LocalBrowserRelayOptions = {
  readonly activeProofTtlMs?: number;
  readonly bootstrapProofTtlMs?: number;
  readonly binding?: { readonly port: number; readonly secret: Buffer };
};
export type LocalAdminRelay = { readonly url: string; stop: () => Promise<void> };

const LOCAL_RELAY_MAX_BODY_BYTES = 1024 * 1024;
export const LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES = 4 * 1024 * 1024;
export const LOCAL_RELAY_MAX_IN_FLIGHT = 64;
const LOCAL_RELAY_DEADLINE_MS = 2_000;
export const EXECUTION_TARGET_RELAY_REQUEST_MAX_BYTES = 8 * 1024;
export const EXECUTION_TARGET_RELAY_MANIFEST_MAX_BYTES = 8 * 1024;
export const EXECUTION_TARGET_RELAY_MAX_IN_FLIGHT = 2;
export const EXECUTION_TARGET_RELAY_DEADLINE_MS = 9_000;
const BROWSER_BROKER_PROOF_DOMAIN = "semio/browser-broker-proof/v1\0";
export const BROWSER_BROKER_PROOF_TTL_MS = 15_000;
export const BROWSER_BROKER_BOOTSTRAP_PROOF_TTL_MS = 120_000;
const ADMIN_RELAY_BOOTSTRAP_PROOF_TTL_MS = 15_000;
const ADMIN_RELAY_SESSION_TTL_MS = 30 * 60_000;
export const ADMIN_RELAY_COOKIE = "semio_admin_relay";
const ADMIN_RELAY_PROOF_DOMAIN = "semio/admin-relay-proof/v1\0";
const ADMIN_RELAY_COOKIE_DOMAIN = "semio/admin-relay-cookie/v1\0";

export function browserBrokerProofDigest(proof: Uint8Array): Buffer {
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

export function startLocalAdminRelay(hubOrigin: string, envelope: Record<string, any>, bootstrapProof: Buffer, proofTtlMs = ADMIN_RELAY_BOOTSTRAP_PROOF_TTL_MS, sessionTtlMs = ADMIN_RELAY_SESSION_TTL_MS): LocalAdminRelay {
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

export async function readLocalRelayBody(request: Request, maximumBytes = LOCAL_RELAY_MAX_BODY_BYTES, signal?: AbortSignal): Promise<Uint8Array<ArrayBuffer> | undefined> {
  signal?.throwIfAborted();
  if (request.method === "GET" || request.body === null) return undefined;
  const contentLength = Number(request.headers.get("content-length") ?? "0");
  if (!Number.isSafeInteger(contentLength) || contentLength < 0 || contentLength > maximumBytes) throw new Error("payload too large");
  const reader = request.body.getReader();
  const cancel = (): void => {
    void reader.cancel().catch(() => undefined);
  };
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

async function readLocalRelayResponse(response: Response, maximumBytes = LOCAL_RELAY_MAX_BODY_BYTES): Promise<Uint8Array<ArrayBuffer>> {
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

/** 🔐️ `crypto.subtle` accepts only `ArrayBuffer`-backed views, while a `Uint8Array`/`Buffer` may be typed over
 * `ArrayBufferLike` (which includes `SharedArrayBuffer`). Every WebCrypto digest here is therefore taken over an
 * exact non-shared copy of the bytes. */
export async function webSha256Hex(bytes: Uint8Array | ArrayBuffer): Promise<string> {
  return Buffer.from(await crypto.subtle.digest("SHA-256", new Uint8Array(bytes))).toString("hex");
}

function matchesSecret(supplied: string | null, expected: Buffer): boolean {
  if (supplied === null || !/^[0-9a-f]{64}$/u.test(supplied)) return false;
  const candidate = Buffer.from(supplied, "hex");
  const matches = candidate.length === expected.length && timingSafeEqual(candidate, expected);
  candidate.fill(0);
  return matches;
}

export function startLocalBrowserRelay(hubOrigin: string, uiOrigin: string, envelope: Record<string, any>, options: LocalBrowserRelayOptions = {}): LocalBrowserRelay {
  const activeProofTtlMs = options.activeProofTtlMs ?? BROWSER_BROKER_PROOF_TTL_MS;
  const bootstrapProofTtlMs = options.bootstrapProofTtlMs ?? BROWSER_BROKER_BOOTSTRAP_PROOF_TTL_MS;
  if (!Number.isSafeInteger(activeProofTtlMs) || activeProofTtlMs <= 0 || activeProofTtlMs > BROWSER_BROKER_PROOF_TTL_MS) throw new Error("browser broker active proof TTL invalid");
  if (!Number.isSafeInteger(bootstrapProofTtlMs) || bootstrapProofTtlMs <= 0 || bootstrapProofTtlMs > BROWSER_BROKER_BOOTSTRAP_PROOF_TTL_MS) throw new Error("browser broker bootstrap proof TTL invalid");
  if (envelope.schema !== "semio.hub.local-credential-envelope/v1" || envelope.clientClass !== "react-relay" || typeof envelope.capability !== "string" || !/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(envelope.capability))
    throw new Error("react relay credential envelope binding mismatch");
  const secret = options.binding?.secret ?? randomBytes(32);
  let browserProofDigest: Buffer | undefined;
  let browserProofExpiresAtMs = 0;
  let browserProofPhase: "unarmed" | "bootstrap" | "active" = "unarmed";
  let bootstrapProofTaken = false;
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
    port: options.binding?.port ?? 0,
    async fetch(request, relayServer): Promise<Response> {
      const supplied = request.headers.get("x-semio-local-relay");
      const origin = request.headers.get("origin");
      const referer = request.headers.get("referer");
      const fetchSite = request.headers.get("sec-fetch-site");
      const host = request.headers.get("host");
      const peer = relayServer.requestIP(request)?.address;
      const rejection = stopping
        ? "stopping"
        : !capability
          ? "capability"
          : !matchesSecret(supplied, secret)
            ? "secret"
            : host !== new URL(uiOrigin).host
              ? "host"
              : peer !== "127.0.0.1" && peer !== "::1"
                ? "peer"
                : origin !== uiOrigin
                  ? "origin"
                  : referer === null || !referer.startsWith(`${uiOrigin}/`)
                    ? "referer"
                    : fetchSite !== "same-origin"
                      ? "fetch-site"
                      : "";
      if (rejection) {
        return new Response("unauthorized", { status: 401 });
      }
      const url = new URL(request.url);
      const upstreamPath = localRelayUpstreamPath(request.method, url);
      if (!upstreamPath || inFlight >= LOCAL_RELAY_MAX_IN_FLIGHT) return new Response("unavailable", { status: upstreamPath ? 503 : 404 });
      const executionTarget = localRelayExecutionTargetAsset(upstreamPath);
      const spaceArtifactCreation = localRelaySpaceArtifactCreationPath(request.method, upstreamPath);
      const inference = localRelayInferencePath(request.method, upstreamPath);
      const spaceArtifactCreationCatalog = request.method === "GET" && /^\/spaces\/[^/]+\/artifact-creations$/u.test(upstreamPath);
      if (executionTarget && executionTargetsInFlight >= EXECUTION_TARGET_RELAY_MAX_IN_FLIGHT) return new Response("unavailable", { status: 503 });
      const requestMaxBytes = executionTarget ? EXECUTION_TARGET_RELAY_REQUEST_MAX_BYTES : spaceArtifactCreation ? SPACE_ARTIFACT_CREATION_MAX_BYTES : inference ? GIS_MAP_INFERENCE_REQUEST_MAX_BYTES : LOCAL_RELAY_MAX_BODY_BYTES;
      const responseMaxBytes =
        executionTarget === "browser-actor"
          ? DOCUMENT_BROWSER_ACTOR_MAX_BYTES
          : executionTarget === "component"
            ? DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES
            : executionTarget === "descriptor"
              ? DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES
              : executionTarget === "manifest"
                ? EXECUTION_TARGET_RELAY_MANIFEST_MAX_BYTES
                : spaceArtifactCreationCatalog
                  ? SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES
                  : spaceArtifactCreation
                  ? SPACE_ARTIFACT_CREATION_MAX_BYTES
                  : inference
                    ? GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES
                    : LOCAL_RELAY_MAX_BODY_BYTES;
      const contentLength = Number(request.headers.get("content-length") ?? "0");
      if (!Number.isSafeInteger(contentLength) || contentLength < 0 || contentLength > requestMaxBytes) return new Response("payload too large", { status: 413 });
      const currentProof = request.headers.get("x-semio-browser-broker");
      const nextProofDigest = request.headers.get("x-semio-browser-broker-next");
      if (browserProofPhase === "unarmed" || browserProofDigest === undefined || Date.now() > browserProofExpiresAtMs || currentProof === null || nextProofDigest === null || !/^[0-9a-f]{64}$/u.test(currentProof) || !/^[0-9a-f]{64}$/u.test(nextProofDigest)) {
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
      browserProofPhase = "active";
      browserProofExpiresAtMs = Date.now() + activeProofTtlMs;
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
        return new Response(responseBody, {
          status: upstream.status,
          headers: { "x-semio-browser-broker-advanced": "1", "content-length": String(responseBody.byteLength), "cache-control": "no-store", ...(contentType ? { "content-type": contentType } : {}) },
        });
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
    takeBrowserBootstrapProof: () => {
      if (stopping || !capability || bootstrapProofTaken || browserProofPhase !== "unarmed") throw new Error("browser broker bootstrap proof unavailable");
      const proof = randomBytes(32);
      browserProofDigest = browserBrokerProofDigest(proof);
      browserProofExpiresAtMs = Date.now() + bootstrapProofTtlMs;
      browserProofPhase = "bootstrap";
      bootstrapProofTaken = true;
      return proof;
    },
    stop: () => {
      if (stopPromise) return stopPromise;
      stopping = true;
      capability = "";
      secret.fill(0);
      browserProofDigest?.fill(0);
      browserProofDigest = undefined;
      browserProofPhase = "unarmed";
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
