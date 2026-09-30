import { LOCAL_SESSION_BROKER_REQUEST_MAX_BYTES, isLocalSessionTokenV1, LOCAL_SESSION_BROKER_FILE, LOCAL_SESSION_BROKER_SCHEMA, LOCAL_SESSION_SCHEMA, parseLocalSessionBrokerRecordV1, parseLocalSessionRequestV1, parseLocalSessionV1, type LocalSessionBrokerRecordV1, type LocalSessionV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧬️schema/🟦️.ts";

import { randomBytes, timingSafeEqual } from "node:crypto";
import { existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { protectOwnerOnly } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔐️owner-only/🟦️.ts";
import { LOCAL_BOOTSTRAP_DEADLINE_MS, writeLocalFrame } from "../📡️framing/🟦️.ts";
import { authenticatedFrame, LOCAL_BOOTSTRAP_SCHEMA, type LocalClientClass, type LocalProfile, verifyAuthenticatedFrame } from "../🛂authentication/🟦️.ts";
import type { LocalHubRun } from "../🏃️execution/🟦️.ts";

/** 🔐️ Issues one authenticated client-class credential bound to the active local Hub run. */
export async function issueLocalCredential(run: LocalHubRun, profileId: string, clientClass: LocalClientClass, sequence = 2, exchangeId = randomBytes(16).toString("hex")): Promise<Record<string, any>> {
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
  if (envelope.kind === "reject") throw localBootstrapRefusal(run, envelope, exchangeId);
  verifyCredentialEnvelope(run, envelope, profileId, clientClass, exchangeId);
  return envelope;
}

/** 🚫️ The hub's signed answer that it refused one exchange (`resource-limit` inside a busy window,
 * `expired`, `denied`, …). The run stays usable: the next issue takes the next sequence. */
export class LocalBootstrapRefusedError extends Error {
  constructor(readonly code: string, readonly exchangeId: string) {
    super(`local bootstrap refused exchange ${exchangeId}: ${code}`);
  }
}

function localBootstrapRefusal(run: LocalHubRun, reject: Record<string, any>, exchangeId: string): LocalBootstrapRefusedError {
  verifyAuthenticatedFrame(run.channelKey, reject);
  if (reject.schema !== LOCAL_BOOTSTRAP_SCHEMA || reject.runId !== run.runId || reject.exchangeId !== exchangeId || typeof reject.code !== "string") throw new Error("local bootstrap refusal binding mismatch");
  return new LocalBootstrapRefusedError(reject.code, exchangeId);
}

function verifyCredentialEnvelope(run: LocalHubRun, envelope: Record<string, any>, profileId: string, clientClass: LocalClientClass, exchangeId: string): void {
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
  )
    throw new Error("local credential envelope binding mismatch");
}

//#region 🎫️SessionBroker
/** 🎫️ Local development session broker: the ONE process that owns a loopback hub's local-bootstrap pipe hands fresh
 * react-relay sessions to local development UIs (every `s` serve, both two-user rows), so a UI that finds a hub it did
 * not start still signs in with no manual step, and a UI whose 15-minute local session expired gets a new one.
 *
 * Trust boundary: the broker record lives in the hub's own `0700` data root (`0600` file) and carries a random bearer
 * secret; reading it is the same privilege as operating the hub. The broker listens on loopback only and mints only
 * the run's declared local profiles, through the authenticated pipe — never a password, never a network credential path.
 * @see ../🧬️schema/🔣️.json */
/** 🛡️ The admin-relay capability a broker started with an administrator profile keeps `0600` in the same data root — what
 * the hub's operator surfaces and the acceptance gates that read the hub's own state (head sequences, connection census)
 * present — and the sibling file a caller creates to ask for a fresh one (consumed; the capability lives 15 minutes). */
export const LOCAL_ADMIN_CAPABILITY_SCHEMA = "semio.hub.local-admin-capability/v1";
export const LOCAL_ADMIN_CAPABILITY_FILE = "admin-capability.json";
export const LOCAL_ADMIN_REQUEST_FILE = "admin-request";
const LOCAL_ADMIN_REQUEST_POLL_MS = 5_000;

export type LocalAdminCapabilityV1 = Readonly<{ schema: typeof LOCAL_ADMIN_CAPABILITY_SCHEMA; origin: string; capability: string; sessionId: string; expiresAt: number }>;
export type LocalSessionBrokerV1 = Readonly<{ record: LocalSessionBrokerRecordV1; stop: () => void }>;

/** 🧾️ Parses one admin capability file exactly. */
export function parseLocalAdminCapabilityV1(value: unknown): LocalAdminCapabilityV1 {
  const file = value as Record<string, unknown> | null;
  if (
    file === null ||
    typeof file !== "object" ||
    Object.keys(file).sort().join(",") !== "capability,expiresAt,origin,schema,sessionId" ||
    file.schema !== LOCAL_ADMIN_CAPABILITY_SCHEMA ||
    typeof file.origin !== "string" ||
    !/^http:\/\/127\.0\.0\.1:\d{1,5}$/u.test(file.origin) ||
    typeof file.capability !== "string" ||
    !isLocalSessionTokenV1(file.capability) ||
    typeof file.sessionId !== "string" ||
    file.sessionId.length === 0 ||
    file.sessionId.length > 256 ||
    !Number.isSafeInteger(file.expiresAt) ||
    (file.expiresAt as number) < 1
  )
    throw new Error("local admin capability invalid");
  return Object.freeze({ ...(file as LocalAdminCapabilityV1) });
}

/** 🔐️ Starts the broker beside one owned run: issues credentials through the run's pipe one at a time, from
 * `firstSequence` on (the pipe's sequence counter belongs to its owner), and writes the record `0600` into `dataDir`.
 * With `adminProfileId` (a profile allowed `admin-relay`, whose subject the run declared an admin subject) it also keeps
 * the {@link LOCAL_ADMIN_CAPABILITY_FILE} there: issued once the broker starts and again whenever a caller creates
 * {@link LOCAL_ADMIN_REQUEST_FILE}; `stop` removes the file it wrote. */
export function startLocalSessionBroker(run: LocalHubRun, dataDir: string, profiles: readonly LocalProfile[], firstSequence: number, adminProfileId?: string): LocalSessionBrokerV1 {
  const hubOrigin = `http://127.0.0.1:${run.port}`;
  const secret = randomBytes(32).toString("hex");
  const secretBytes = Buffer.from(secret, "hex");
  const profileIds = profiles.filter((profile) => profile.allowedClientClasses.includes("react-relay")).map((profile) => profile.profileId);
  if (adminProfileId !== undefined && !profiles.some((profile) => profile.profileId === adminProfileId && profile.allowedClientClasses.includes("admin-relay"))) throw new Error(`local session broker: ${adminProfileId} is no admin-relay profile of this run`);
  let sequence = firstSequence;
  let tail: Promise<unknown> = Promise.resolve();
  const adminPath = join(dataDir, LOCAL_ADMIN_CAPABILITY_FILE);
  const adminRequestPath = join(dataDir, LOCAL_ADMIN_REQUEST_FILE);
  let adminSessionId: string | null = null;
  const issueAdmin = (profileId: string): Promise<void> => {
    const work = tail.then(async () => {
      const envelope = await issueLocalCredential(run, profileId, "admin-relay", sequence++);
      const file = parseLocalAdminCapabilityV1({ schema: LOCAL_ADMIN_CAPABILITY_SCHEMA, origin: hubOrigin, capability: String(envelope.capability ?? ""), sessionId: String(envelope.sessionId ?? ""), expiresAt: Number(envelope.expiresAt) });
      writeFileSync(adminPath, `${JSON.stringify(file)}\n`, { mode: 0o600 });
      protectOwnerOnly(adminPath, "file");
      adminSessionId = file.sessionId;
    });
    tail = work.catch(() => undefined);
    return work;
  };
  const adminRequests =
    adminProfileId === undefined
      ? null
      : setInterval(() => {
          if (!existsSync(adminRequestPath)) return;
          rmSync(adminRequestPath, { force: true });
          void issueAdmin(adminProfileId).catch(() => undefined);
        }, LOCAL_ADMIN_REQUEST_POLL_MS);
  if (adminProfileId !== undefined) void issueAdmin(adminProfileId).catch(() => undefined);
  const issue = (profileId: string): Promise<LocalSessionV1> => {
    const work = tail.then(async () => {
      const envelope = await issueLocalCredential(run, profileId, "react-relay", sequence++);
      const token = String(envelope.capability ?? "");
      const me = await fetch(`${hubOrigin}/auth/sessions/me`, { headers: { authorization: `Bearer ${token}` }, signal: AbortSignal.timeout(10_000) });
      const body = (await me.json()) as { readonly user_id?: string; readonly userId?: string };
      return parseLocalSessionV1({ schema: LOCAL_SESSION_SCHEMA, profileId, token, userId: body.userId ?? body.user_id ?? "" });
    });
    tail = work.catch(() => undefined);
    return work;
  };
  const server = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request) {
      if (new URL(request.url).pathname !== "/session" || request.method !== "POST") return new Response(null, { status: 404 });
      const bearer = Buffer.from((request.headers.get("authorization") ?? "").replace(/^Bearer /u, ""), "hex");
      if (bearer.length !== secretBytes.length || !timingSafeEqual(bearer, secretBytes)) return new Response(null, { status: 401 });
      const text = await request.text();
      if (text.length > LOCAL_SESSION_BROKER_REQUEST_MAX_BYTES) return new Response(null, { status: 413 });
      let profileId: string;
      try {
        profileId = parseLocalSessionRequestV1(JSON.parse(text));
      } catch {
        return new Response(null, { status: 400 });
      }
      if (!profileIds.includes(profileId)) return new Response(null, { status: 404 });
      try {
        return Response.json(await issue(profileId), { headers: { "cache-control": "no-store" } });
      } catch {
        return new Response(null, { status: 503 });
      }
    },
  });
  const record = parseLocalSessionBrokerRecordV1({ schema: LOCAL_SESSION_BROKER_SCHEMA, hubOrigin, runId: run.runId, port: server.port, secret, profiles: profileIds });
  const path = join(dataDir, LOCAL_SESSION_BROKER_FILE);
  writeFileSync(path, `${JSON.stringify(record)}\n`, { mode: 0o600 });
  protectOwnerOnly(path, "file");
  return Object.freeze({
    record,
    stop: () => {
      server.stop(true);
      secretBytes.fill(0);
      if (adminRequests !== null) clearInterval(adminRequests);
      try {
        if (adminSessionId !== null && parseLocalAdminCapabilityV1(JSON.parse(readFileSync(adminPath, "utf8"))).sessionId === adminSessionId) rmSync(adminPath, { force: true });
      } catch {
        rmSync(adminPath, { force: true });
      }
      try {
        const current = parseLocalSessionBrokerRecordV1(JSON.parse(readFileSync(path, "utf8")));
        if (current.secret === secret) rmSync(path, { force: true });
      } catch {
        rmSync(path, { force: true });
      }
    },
  });
}

//#endregion 🎫️SessionBroker
