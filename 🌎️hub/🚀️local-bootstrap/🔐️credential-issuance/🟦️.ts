import { randomBytes, timingSafeEqual } from "node:crypto";
import { chmodSync, existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
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
export const LOCAL_SESSION_BROKER_SCHEMA = "semio.hub.local-session-broker/v1";
export const LOCAL_SESSION_REQUEST_SCHEMA = "semio.hub.local-session-request/v1";
export const LOCAL_SESSION_SCHEMA = "semio.hub.local-session/v1";
/** 📄️ The broker record's file name inside the hub data root. */
export const LOCAL_SESSION_BROKER_FILE = "local-session-broker.json";
const LOCAL_SESSION_BROKER_REQUEST_MAX_BYTES = 256;
const LOCAL_SESSION_TOKEN_PATTERN = /^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u;
const LOCAL_PROFILE_ID_PATTERN = /^[a-z0-9](?:[a-z0-9.-]*[a-z0-9])?$/u;

export type LocalSessionBrokerRecordV1 = Readonly<{ schema: typeof LOCAL_SESSION_BROKER_SCHEMA; hubOrigin: string; runId: string; port: number; secret: string; profiles: readonly string[] }>;
export type LocalSessionV1 = Readonly<{ schema: typeof LOCAL_SESSION_SCHEMA; profileId: string; token: string; userId: string }>;
export type LocalSessionBrokerV1 = Readonly<{ record: LocalSessionBrokerRecordV1; stop: () => void }>;

/** 🧾️ Parses one broker record exactly; anything else is not a broker this launcher trusts. */
export function parseLocalSessionBrokerRecordV1(value: unknown): LocalSessionBrokerRecordV1 {
  const record = value as Record<string, unknown> | null;
  if (
    record === null ||
    typeof record !== "object" ||
    Object.keys(record).sort().join(",") !== "hubOrigin,port,profiles,runId,schema,secret" ||
    record.schema !== LOCAL_SESSION_BROKER_SCHEMA ||
    typeof record.hubOrigin !== "string" ||
    !/^http:\/\/127\.0\.0\.1:\d{1,5}$/u.test(record.hubOrigin) ||
    typeof record.runId !== "string" ||
    !/^[0-9a-f]{32}$/u.test(record.runId) ||
    !Number.isSafeInteger(record.port) ||
    (record.port as number) < 1 ||
    (record.port as number) > 65_535 ||
    typeof record.secret !== "string" ||
    !/^[0-9a-f]{64}$/u.test(record.secret) ||
    !Array.isArray(record.profiles) ||
    record.profiles.length < 1 ||
    record.profiles.length > 8 ||
    record.profiles.some((profile) => typeof profile !== "string" || profile.length > 64 || !LOCAL_PROFILE_ID_PATTERN.test(profile))
  )
    throw new Error("local session broker record invalid");
  return Object.freeze({ ...(record as LocalSessionBrokerRecordV1), profiles: Object.freeze([...(record.profiles as string[])]) });
}

/** 🧾️ Parses one broker request exactly and answers its profile id. */
export function parseLocalSessionRequestV1(value: unknown): string {
  const request = value as Record<string, unknown> | null;
  if (request === null || typeof request !== "object" || Object.keys(request).sort().join(",") !== "profileId,schema" || request.schema !== LOCAL_SESSION_REQUEST_SCHEMA || typeof request.profileId !== "string" || !LOCAL_PROFILE_ID_PATTERN.test(request.profileId) || request.profileId.length > 64)
    throw new Error("local session request invalid");
  return request.profileId;
}

/** 🧾️ Parses one issued session exactly. */
export function parseLocalSessionV1(value: unknown): LocalSessionV1 {
  const session = value as Record<string, unknown> | null;
  if (
    session === null ||
    typeof session !== "object" ||
    Object.keys(session).sort().join(",") !== "profileId,schema,token,userId" ||
    session.schema !== LOCAL_SESSION_SCHEMA ||
    typeof session.profileId !== "string" ||
    session.profileId.length > 64 ||
    !LOCAL_PROFILE_ID_PATTERN.test(session.profileId) ||
    typeof session.token !== "string" ||
    !LOCAL_SESSION_TOKEN_PATTERN.test(session.token) ||
    typeof session.userId !== "string" ||
    session.userId.length === 0 ||
    session.userId.length > 128
  )
    throw new Error("local session invalid");
  return Object.freeze({ ...(session as LocalSessionV1) });
}

/** 🔐️ Starts the broker beside one owned run: issues credentials through the run's pipe one at a time, from
 * `firstSequence` on (the pipe's sequence counter belongs to its owner), and writes the record `0600` into `dataDir`. */
export function startLocalSessionBroker(run: LocalHubRun, dataDir: string, profiles: readonly LocalProfile[], firstSequence: number): LocalSessionBrokerV1 {
  const hubOrigin = `http://127.0.0.1:${run.port}`;
  const secret = randomBytes(32).toString("hex");
  const secretBytes = Buffer.from(secret, "hex");
  const profileIds = profiles.filter((profile) => profile.allowedClientClasses.includes("react-relay")).map((profile) => profile.profileId);
  let sequence = firstSequence;
  let tail: Promise<unknown> = Promise.resolve();
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
  chmodSync(path, 0o600);
  return Object.freeze({
    record,
    stop: () => {
      server.stop(true);
      secretBytes.fill(0);
      try {
        const current = parseLocalSessionBrokerRecordV1(JSON.parse(readFileSync(path, "utf8")));
        if (current.secret === secret) rmSync(path, { force: true });
      } catch {
        rmSync(path, { force: true });
      }
    },
  });
}

/** 🎫️ Asks the live hub's broker (the record in `dataDir` whose `runId` the hub at `hubOrigin` answers `/readyz` with)
 * for a fresh session of `profileId`. `null` when that hub has no live broker — a hub someone else operates. */
export async function requestLocalBrokerSession(dataDir: string, hubOrigin: string, profileId: string, signal: AbortSignal = AbortSignal.timeout(20_000)): Promise<LocalSessionV1 | null> {
  const path = join(dataDir, LOCAL_SESSION_BROKER_FILE);
  if (!existsSync(path)) return null;
  let record: LocalSessionBrokerRecordV1;
  try {
    record = parseLocalSessionBrokerRecordV1(JSON.parse(readFileSync(path, "utf8")));
  } catch {
    return null;
  }
  if (record.hubOrigin !== hubOrigin.replace(/\/+$/u, "") || !record.profiles.includes(profileId)) return null;
  const readiness = (await fetch(`${record.hubOrigin}/readyz`, { signal }).then((response) => response.json()).catch(() => null)) as { readonly runId?: unknown } | null;
  if (readiness?.runId !== record.runId) return null;
  const response = await fetch(`http://127.0.0.1:${record.port}/session`, {
    method: "POST",
    headers: { authorization: `Bearer ${record.secret}`, "content-type": "application/json" },
    body: JSON.stringify({ schema: LOCAL_SESSION_REQUEST_SCHEMA, profileId }),
    signal,
  }).catch(() => null);
  if (response === null || !response.ok) return null;
  return parseLocalSessionV1(await response.json());
}
//#endregion 🎫️SessionBroker
