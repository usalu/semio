/** 🔐️ Hub sign-in contract — the browser-facing half of `POST /auth/sessions` (the session-mint
 * route whose Rust response twin is `SessionMintResponse` in `../🔌️client/🦀️.rs`) plus the
 * local-only hub connection book the shell offers before any credential is typed. Pure: no `fetch`,
 * no React, no storage handle of its own — every side effect enters through the injected
 * {@link HubSignInTransportV1}/{@link HubConnectionStorageV1} ports, exactly as
 * `../🪪️session-refresh/🟦️.ts` injects its `read`. The sign-in *token* is never written to any
 * store: {@link HubConnectionBookV1} persists hub identity only (origin, label, kind, last user
 * hint), so a stolen profile directory yields no capability. */

import type { DirectorySpaceRole } from "../🧬️schema/🟦️.ts";

//#region 🔖️Routes
export const HUB_SESSION_MINT_PATH_V1 = "/auth/sessions";
export const HUB_SESSION_ME_PATH_V1 = "/auth/sessions/me";
export const HUB_SIGN_IN_REQUEST_SCHEMA_V1 = "semio.hub.auth.credential-sign-in/v1";
export const HUB_AUTH_ERROR_SCHEMA_V1 = "semio.hub.auth.error/v1";
/** 📏️ AU1 §1.1: axum's `DefaultBodyLimit` rejects a larger body with 413 before the handler runs. */
export const HUB_SESSION_MINT_REQUEST_MAX_BYTES = 1024;
export const HUB_SESSION_MINT_RESPONSE_MAX_BYTES = 8 * 1024;
/** 🪪️ `DIRECTORY_SESSION_AUTHORITY_MAX_BYTES` in `📇️directory/🧬️schema/🪪️session-authority-v1/🦀️.rs`. */
export const HUB_SESSION_AUTHORITY_SCHEMA_V1 = "semio.directory.session-authority.v1";
export const HUB_SESSION_AUTHORITY_MAX_BYTES = 2048;
export const HUB_SIGN_IN_TIMEOUT_MS = 10_000;
export const HUB_SIGN_IN_RATE_LIMIT_MAX_SECONDS = 24 * 60 * 60;
export const HUB_SIGN_IN_EMAIL_MIN_BYTES = 3;
export const HUB_SIGN_IN_EMAIL_MAX_BYTES = 254;
export const HUB_SIGN_IN_PASSWORD_MIN_BYTES = 8;
export const HUB_SIGN_IN_PASSWORD_MAX_BYTES = 256;
export const HUB_SIGN_IN_DEVICE_INSTANCE_MAX_BYTES = 128;
//#endregion 🔖️Routes

//#region 🔖️Text
/** 🌐️ English first, German second, no default language — an unowned locale is refused rather than
 * silently downgraded (`AGENTS.md`: "multiple languages with no default language"). */
export const HUB_SIGN_IN_TEXT_V1 = {
  en: {
    invalidCredentials: "That email and password do not match an account on this hub.",
    rateLimited: "Too many sign-in attempts. Try again in {{seconds}} s.",
    unreachable: "This hub cannot be reached. You can keep working locally; sign in again when the connection is back.",
    hubRefused: "This hub refused the sign-in request.",
    invalidResponse: "This hub answered with something this app cannot read.",
    cancelled: "Sign-in was cancelled.",
    expired: "Your session on this hub expired. Sign in again to continue collaborating.",
    invalidOrigin: "Enter a hub address such as https://hub.example.org.",
    passwordDisabled: "This hub does not accept password sign-in. Open it from its own launcher instead.",
    malformedRequest: "This app sent something this hub could not read. Check the address and try again.",
    shortPassword: "Passwords on this hub are at least 8 characters.",
    invalidEmail: "Enter the email address you use on this hub.",
  },
  de: {
    invalidCredentials: "E-Mail und Passwort passen zu keinem Konto auf diesem Hub.",
    rateLimited: "Zu viele Anmeldeversuche. Versuche es in {{seconds}} s erneut.",
    unreachable: "Dieser Hub ist nicht erreichbar. Du kannst lokal weiterarbeiten und dich erneut anmelden, sobald die Verbindung wieder steht.",
    hubRefused: "Dieser Hub hat die Anmeldung abgelehnt.",
    invalidResponse: "Dieser Hub hat mit etwas geantwortet, das diese App nicht lesen kann.",
    cancelled: "Die Anmeldung wurde abgebrochen.",
    expired: "Deine Sitzung auf diesem Hub ist abgelaufen. Melde dich erneut an, um weiter zusammenzuarbeiten.",
    invalidOrigin: "Gib eine Hub-Adresse ein, zum Beispiel https://hub.example.org.",
    passwordDisabled: "Dieser Hub akzeptiert keine Passwort-Anmeldung. Öffne ihn stattdessen über seinen eigenen Starter.",
    malformedRequest: "Diese App hat etwas gesendet, das dieser Hub nicht lesen konnte. Prüfe die Adresse und versuche es erneut.",
    shortPassword: "Passwörter auf diesem Hub haben mindestens 8 Zeichen.",
    invalidEmail: "Gib die E-Mail-Adresse ein, die du auf diesem Hub verwendest.",
  },
} as const;

export type HubSignInLocaleV1 = keyof typeof HUB_SIGN_IN_TEXT_V1;

/** 🔡️ Refuses unowned locales instead of silently selecting a default language. */
export function hubSignInTextV1(locale: string): (typeof HUB_SIGN_IN_TEXT_V1)[HubSignInLocaleV1] {
  if (locale !== "en" && locale !== "de") throw new Error("hub.sign-in.locale-unsupported");
  return HUB_SIGN_IN_TEXT_V1[locale];
}
//#endregion 🔖️Text

//#region 🔖️Errors
/** 🚫️ Closed sign-in denial classes. Every one of them is separately rendered by `🔐️HubSignIn`, so
 * a human never sees a bare status code. `invalid-credentials` is deliberately one class covering
 * unknown email, no password credential and wrong password — AU1 §1.1 emits one uniform code so the
 * route cannot be used to enumerate users, and the UI must not try to distinguish. */
export type HubSignInErrorCodeV1 = "invalid-credentials" | "rate-limited" | "unreachable" | "hub-refused" | "invalid-response" | "cancelled" | "password-sign-in-disabled" | "malformed-request";

const HUB_SIGN_IN_TEXT_KEY: Readonly<Record<HubSignInErrorCodeV1, keyof (typeof HUB_SIGN_IN_TEXT_V1)["en"]>> = {
  "invalid-credentials": "invalidCredentials",
  "rate-limited": "rateLimited",
  unreachable: "unreachable",
  "hub-refused": "hubRefused",
  "invalid-response": "invalidResponse",
  cancelled: "cancelled",
  "password-sign-in-disabled": "passwordDisabled",
  "malformed-request": "malformedRequest",
};

/** 🗣️ Resolves one denial class to its locale text, substituting the rate-limit countdown. */
export function hubSignInErrorTextV1(locale: string, code: HubSignInErrorCodeV1, retryAfterSeconds: number | null): string {
  const text = hubSignInTextV1(locale)[HUB_SIGN_IN_TEXT_KEY[code]];
  return code === "rate-limited" ? text.replace("{{seconds}}", String(retryAfterSeconds ?? 0)) : text;
}

/** 🌐️ Maps one mint status to its closed code, per AU1 §1.1's status table: 400
 * `malformed-request`, 401 `invalid-credentials`, 403 `credential-sign-in-disabled`, 429
 * `rate-limited`, 503 `directory-unavailable`, plus 408/5xx as unreachable and 413 (the body-limit
 * rejection that never reaches the handler) as malformed. */
export function hubSignInErrorFromStatusV1(status: number): HubSignInErrorCodeV1 {
  if (status === 400 || status === 413 || status === 415) return "malformed-request";
  if (status === 401) return "invalid-credentials";
  if (status === 403) return "password-sign-in-disabled";
  if (status === 429) return "rate-limited";
  if (status === 408 || status >= 500) return "unreachable";
  return "hub-refused";
}

/** 🏷️ AU1 §1.1's `semio.hub.auth.error/v1` body. Read only to recover `retryAfterMs` when the
 * `retry-after` header is missing; the status is always the authority for the code, so a hub that
 * contradicts itself cannot widen the UI's error vocabulary. */
export function hubAuthErrorRetryAfterSecondsV1(body: string): number | null {
  let value: unknown;
  try {
    value = JSON.parse(body);
  } catch {
    return null;
  }
  if (value === null || typeof value !== "object" || Array.isArray(value)) return null;
  const object = value as Record<string, unknown>;
  if (object.schema !== HUB_AUTH_ERROR_SCHEMA_V1) return null;
  const ms = object.retryAfterMs;
  if (typeof ms !== "number" || !Number.isSafeInteger(ms) || ms < 0) return null;
  const seconds = Math.ceil(ms / 1000);
  return seconds <= HUB_SIGN_IN_RATE_LIMIT_MAX_SECONDS ? seconds : null;
}

/** ⏳️ Reads a `Retry-After` delta-seconds header, clamped to a day; an HTTP-date form or any
 * non-integer yields `null` rather than a fabricated countdown. */
export function hubRetryAfterSecondsV1(header: string | null | undefined): number | null {
  if (typeof header !== "string" || !/^[0-9]{1,6}$/u.test(header.trim())) return null;
  const seconds = Number.parseInt(header.trim(), 10);
  return seconds >= 0 && seconds <= HUB_SIGN_IN_RATE_LIMIT_MAX_SECONDS ? seconds : null;
}
//#endregion 🔖️Errors

//#region 🔖️MintWire
/** 🎫️ `POST /auth/sessions`'s response. Wire is snake_case (`token`, `user_id`) — the Rust twin
 * `SessionMintResponse` documents that this route predates the camelCase wave, and this parser
 * matches the actual wire rather than the convention. */
export interface HubSessionMintResultV1 {
  readonly token: string;
  readonly userId: string;
}

/** 🔐️ The exact credential body posted to the mint route. Held only for the duration of one
 * request; never stored, never logged, never placed in a URL. */
export interface HubSignInCredentialV1 {
  readonly email: string;
  readonly password: string;
  readonly deviceInstanceId: string;
  readonly clientClass: HubSignInClientClassV1;
}

export type HubSignInClientClassV1 = "browser" | "native" | "cli";

/** 🎫️ AU1 §1.1's token shape `session.v1.<32 lower-hex>.<64 lower-hex>`. Pinned exactly, so a proxy
 * login page or an error string can never be installed as a session capability. */
const HUB_SESSION_TOKEN = /^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u;
const HUB_DEVICE_INSTANCE_ID = /^[A-Za-z0-9._:-]+$/u;

/** 📥️ Parses exactly `{token, user_id}` — an extra field, a short token or a control character is a
 * refusal, so a proxy's error page can never be mistaken for a session. */
export function parseHubSessionMintResultV1(source: string): HubSessionMintResultV1 {
  if (new TextEncoder().encode(source).byteLength > HUB_SESSION_MINT_RESPONSE_MAX_BYTES) throw new Error("hub.sign-in.response-too-large");
  const value: unknown = JSON.parse(source);
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("hub.sign-in.invalid-response");
  const object = value as Record<string, unknown>;
  const keys = Object.keys(object).sort();
  if (keys.length !== 2 || keys[0] !== "token" || keys[1] !== "user_id") throw new Error("hub.sign-in.invalid-response");
  const token = object.token;
  const userId = object.user_id;
  if (typeof token !== "string" || !HUB_SESSION_TOKEN.test(token)) throw new Error("hub.sign-in.invalid-response");
  if (typeof userId !== "string" || userId.length === 0 || userId.length > 256 || /\p{Cc}/u.test(userId)) throw new Error("hub.sign-in.invalid-response");
  return { token, userId };
}

/** ✉️ The hub's own `$defs/CredentialSignInRequestV1.email` admission, mirrored here so an obviously
 * wrong field never costs a round trip against a rate-limited route: 3..=254 bytes and the exact
 * pattern `^[^ - @]+@[^ - @]+$`
 * (`🌎️hub/🔐️auth/🧬️schema/🔣️.json`). */
export const HUB_SIGN_IN_EMAIL_PATTERN_V1 = /^[^ - @]+@[^ - @]+$/u;

export function validHubSignInEmailV1(email: string): boolean {
  const bytes = new TextEncoder().encode(email).byteLength;
  return bytes >= HUB_SIGN_IN_EMAIL_MIN_BYTES && bytes <= HUB_SIGN_IN_EMAIL_MAX_BYTES && HUB_SIGN_IN_EMAIL_PATTERN_V1.test(email);
}

/** 🔑️ AU1 §1.1's password admission: 8..=256 bytes, no control bytes. */
export function validHubSignInPasswordV1(password: string): boolean {
  const bytes = new TextEncoder().encode(password).byteLength;
  return bytes >= HUB_SIGN_IN_PASSWORD_MIN_BYTES && bytes <= HUB_SIGN_IN_PASSWORD_MAX_BYTES && !/\p{Cc}/u.test(password);
}

/** 📤️ Builds the exact mint body in AU1 §1.1's declared field order. Rejects an out-of-bounds field
 * before it reaches the wire so a malformed credential is a local error, not a hub round trip that
 * also spends a token from the sign-in bucket. */
export function hubSessionMintRequestJsonV1(credential: HubSignInCredentialV1): string {
  if (!validHubSignInEmailV1(credential.email.trim())) throw new Error("hub.sign-in.invalid-email");
  if (!validHubSignInPasswordV1(credential.password)) throw new Error("hub.sign-in.invalid-password");
  const device = credential.deviceInstanceId;
  if (device.length === 0 || new TextEncoder().encode(device).byteLength > HUB_SIGN_IN_DEVICE_INSTANCE_MAX_BYTES || !HUB_DEVICE_INSTANCE_ID.test(device)) throw new Error("hub.sign-in.invalid-device");
  const body = JSON.stringify({
    schema: HUB_SIGN_IN_REQUEST_SCHEMA_V1,
    email: credential.email.trim().toLowerCase(),
    password: credential.password,
    deviceInstanceId: device,
    clientClass: credential.clientClass,
  });
  if (new TextEncoder().encode(body).byteLength > HUB_SESSION_MINT_REQUEST_MAX_BYTES) throw new Error("hub.sign-in.request-too-large");
  return body;
}
//#endregion 🔖️MintWire

//#region 🔖️Device
const HUB_DEVICE_INSTANCE_STORAGE_KEY_V1 = "semio.os.hub-device-instance.v1";

/** 🖥️ This browser profile's stable device instance id — a random opaque token, never a fingerprint
 * and never derived from anything about the human. Persisted local-only so one device keeps one
 * session lineage across reloads; a blocked or throwing store yields a per-session id instead of
 * failing, because a sign-in must never depend on storage being writable. */
export function hubDeviceInstanceIdV1(storage: HubConnectionStorageV1 | null, random: () => string = () => Array.from(crypto.getRandomValues(new Uint8Array(16)), (byte) => byte.toString(16).padStart(2, "0")).join("")): string {
  try {
    const existing = storage?.getItem(HUB_DEVICE_INSTANCE_STORAGE_KEY_V1);
    if (typeof existing === "string" && existing.length > 0 && existing.length <= HUB_SIGN_IN_DEVICE_INSTANCE_MAX_BYTES && /^[A-Za-z0-9._:-]+$/u.test(existing)) return existing;
    const minted = random();
    storage?.setItem(HUB_DEVICE_INSTANCE_STORAGE_KEY_V1, minted);
    return minted;
  } catch {
    return random();
  }
}
//#endregion 🔖️Device

//#region 🔖️Connections
export type HubConnectionKindV1 = "local-bootstrap" | "remote";

/** 🏛️ One hub this device knows about. `origin` is a bare scheme+authority — no path, query,
 * fragment or embedded credentials — so it can never smuggle a capability through the book. */
export interface HubConnectionV1 {
  readonly id: string;
  readonly label: string;
  readonly origin: string;
  readonly kind: HubConnectionKindV1;
  readonly lastUserId: string | null;
}

export const HUB_CONNECTION_BOOK_SCHEMA_V1 = "semio.os.hub-connection-book.v1";
export const HUB_CONNECTION_BOOK_STORAGE_KEY_V1 = "semio.os.hub-connection-book.v1";
export const HUB_CONNECTION_BOOK_MAX_ENTRIES = 16;
export const LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1 = "local-bootstrap";

/** 📕️ The persisted-local-only hub book: which hubs this browser profile knows and which one is
 * selected. Contains no token, no password and no session id — only hub identity. */
export interface HubConnectionBookV1 {
  readonly schema: typeof HUB_CONNECTION_BOOK_SCHEMA_V1;
  readonly connections: readonly HubConnectionV1[];
  readonly selectedId: string;
}

/** 🗄️ The one storage capability this module needs — a `Storage`-shaped port, so a test passes a
 * map and the shell passes `localStorage` without this file naming a browser global. */
export interface HubConnectionStorageV1 {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
}

/** 🏠️ The always-present local bootstrap entry: the same-origin hub the dev launcher inherits a
 * credential for. Kept first so an offline device still has a usable selection. */
export function localBootstrapHubConnectionV1(origin: string): HubConnectionV1 {
  return { id: LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1, label: "This device", origin, kind: "local-bootstrap", lastUserId: null };
}

/** 🌐️ Normalizes typed hub text to a bare origin. Adds `https://` when no scheme is typed, refuses
 * anything carrying a path, query, fragment, username or password. */
export function parseHubOriginV1(text: string): string {
  const trimmed = text.trim();
  if (trimmed.length === 0 || trimmed.length > 512 || /\p{Cc}|\s/u.test(trimmed)) throw new Error("hub.sign-in.invalid-origin");
  const candidate = /^[a-z][a-z0-9+.-]*:\/\//iu.test(trimmed) ? trimmed : `https://${trimmed}`;
  let url: URL;
  try {
    url = new URL(candidate);
  } catch {
    throw new Error("hub.sign-in.invalid-origin");
  }
  if (url.protocol !== "https:" && url.protocol !== "http:") throw new Error("hub.sign-in.invalid-origin");
  if (url.username.length > 0 || url.password.length > 0 || url.search.length > 0 || url.hash.length > 0) throw new Error("hub.sign-in.invalid-origin");
  if (url.pathname !== "/" && url.pathname.length > 0) throw new Error("hub.sign-in.invalid-origin");
  if (url.hostname.length === 0) throw new Error("hub.sign-in.invalid-origin");
  return url.origin;
}

/** 🆔️ A stable, collision-free id for a remote hub: its own origin. Two different labels for one
 * origin are the same hub, and re-adding it updates rather than duplicates. */
export function hubConnectionIdForOriginV1(origin: string): string {
  return `remote:${origin}`;
}

function validHubConnection(value: unknown): value is HubConnectionV1 {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
  const object = value as Record<string, unknown>;
  const keys = Object.keys(object).sort().join(",");
  if (keys !== "id,kind,label,lastUserId,origin") return false;
  if (typeof object.id !== "string" || object.id.length === 0 || object.id.length > 600) return false;
  if (typeof object.label !== "string" || object.label.length === 0 || object.label.length > 128 || /\p{Cc}/u.test(object.label)) return false;
  if (object.kind !== "local-bootstrap" && object.kind !== "remote") return false;
  if (object.lastUserId !== null && (typeof object.lastUserId !== "string" || object.lastUserId.length === 0 || object.lastUserId.length > 256)) return false;
  if (typeof object.origin !== "string") return false;
  try {
    return parseHubOriginV1(object.origin) === object.origin;
  } catch {
    return false;
  }
}

/** 📖️ Reads the book, repairing rather than throwing: unreadable or tampered storage yields the
 * bootstrap-only book so a corrupt profile never blocks the app from starting locally. */
export function parseHubConnectionBookV1(source: string | null, bootstrapOrigin: string): HubConnectionBookV1 {
  const fallback: HubConnectionBookV1 = { schema: HUB_CONNECTION_BOOK_SCHEMA_V1, connections: [localBootstrapHubConnectionV1(bootstrapOrigin)], selectedId: LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1 };
  if (source === null) return fallback;
  let value: unknown;
  try {
    value = JSON.parse(source);
  } catch {
    return fallback;
  }
  if (value === null || typeof value !== "object" || Array.isArray(value)) return fallback;
  const object = value as Record<string, unknown>;
  if (object.schema !== HUB_CONNECTION_BOOK_SCHEMA_V1 || !Array.isArray(object.connections) || typeof object.selectedId !== "string") return fallback;
  const remotes = object.connections.filter((entry): entry is HubConnectionV1 => validHubConnection(entry) && entry.kind === "remote").slice(0, HUB_CONNECTION_BOOK_MAX_ENTRIES - 1);
  const connections = [localBootstrapHubConnectionV1(bootstrapOrigin), ...remotes];
  const selectedId = connections.some((entry) => entry.id === object.selectedId) ? object.selectedId : LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1;
  return { schema: HUB_CONNECTION_BOOK_SCHEMA_V1, connections, selectedId };
}

/** 💾️ Serializes the book in a fixed key order so two writes of the same state are byte-identical. */
export function serializeHubConnectionBookV1(book: HubConnectionBookV1): string {
  return JSON.stringify({
    schema: book.schema,
    connections: book.connections.filter((entry) => entry.kind === "remote").map((entry) => ({ id: entry.id, kind: entry.kind, label: entry.label, lastUserId: entry.lastUserId, origin: entry.origin })),
    selectedId: book.selectedId,
  });
}

/** 📖️ Loads the book through the injected storage port, tolerating a throwing accessor (private
 * mode, blocked site data) by returning the bootstrap-only book. */
export function readHubConnectionBookV1(storage: HubConnectionStorageV1 | null, bootstrapOrigin: string): HubConnectionBookV1 {
  try {
    return parseHubConnectionBookV1(storage?.getItem(HUB_CONNECTION_BOOK_STORAGE_KEY_V1) ?? null, bootstrapOrigin);
  } catch {
    return parseHubConnectionBookV1(null, bootstrapOrigin);
  }
}

/** 💾️ Persists the book, swallowing a throwing/full store — losing the hub list is a convenience
 * regression, never a reason to fail the sign-in the human just completed. */
export function writeHubConnectionBookV1(storage: HubConnectionStorageV1 | null, book: HubConnectionBookV1): void {
  try {
    storage?.setItem(HUB_CONNECTION_BOOK_STORAGE_KEY_V1, serializeHubConnectionBookV1(book));
  } catch {
    return;
  }
}

/** ➕️ Adds or updates one remote hub and selects it. The bootstrap entry is never replaced, and the
 * book is capped so a scripted loop cannot grow the profile without bound. */
export function upsertHubConnectionV1(book: HubConnectionBookV1, connection: HubConnectionV1): HubConnectionBookV1 {
  if (connection.kind !== "remote") return { ...book, selectedId: LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1 };
  const others = book.connections.filter((entry) => entry.kind === "remote" && entry.id !== connection.id);
  const remotes = [connection, ...others].slice(0, HUB_CONNECTION_BOOK_MAX_ENTRIES - 1);
  return { schema: book.schema, connections: [localBootstrapHubConnectionV1(book.connections[0]!.origin), ...remotes], selectedId: connection.id };
}

/** ➖️ Forgets one remote hub; removing the selected hub falls back to the local bootstrap. */
export function removeHubConnectionV1(book: HubConnectionBookV1, id: string): HubConnectionBookV1 {
  if (id === LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1) return book;
  const connections = book.connections.filter((entry) => entry.id !== id);
  return { schema: book.schema, connections, selectedId: book.selectedId === id ? LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1 : book.selectedId };
}

/** 🎯️ Selects a known hub; an unknown id leaves the book untouched. */
export function selectHubConnectionV1(book: HubConnectionBookV1, id: string): HubConnectionBookV1 {
  return book.connections.some((entry) => entry.id === id) ? { ...book, selectedId: id } : book;
}

/** 🔎️ The currently selected connection — always defined, because the bootstrap entry cannot be
 * removed. */
export function selectedHubConnectionV1(book: HubConnectionBookV1): HubConnectionV1 {
  return book.connections.find((entry) => entry.id === book.selectedId) ?? book.connections[0]!;
}
//#endregion 🔖️Connections

//#region 🔖️SessionState
export type HubSessionPhaseV1 = "signed-out" | "signing-in" | "signed-in" | "expired" | "signing-out";

/** 🪪️ The shell's whole hub-session state. `offline` is orthogonal to `phase`: losing the network
 * while signed in must never sign the human out, because the app keeps working locally. */
export interface HubSessionStateV1 {
  readonly phase: HubSessionPhaseV1;
  readonly connectionId: string;
  readonly userId: string | null;
  readonly expiresAtMs: number | null;
  readonly role: DirectorySpaceRole | null;
  readonly error: HubSignInErrorCodeV1 | null;
  readonly retryAfterSeconds: number | null;
  readonly offline: boolean;
}

/** 📨️ Every transition the session may take. Event-driven, CQRS-shaped: the UI raises an intent,
 * the transport emits a fact, this reducer is the only place state changes. */
export type HubSessionEventV1 =
  | { readonly kind: "select-connection"; readonly connectionId: string }
  | { readonly kind: "submit" }
  | { readonly kind: "minted"; readonly userId: string; readonly expiresAtMs: number | null }
  | { readonly kind: "failed"; readonly code: HubSignInErrorCodeV1; readonly retryAfterSeconds: number | null }
  | { readonly kind: "expired" }
  | { readonly kind: "sign-out" }
  | { readonly kind: "signed-out" }
  | { readonly kind: "connectivity"; readonly offline: boolean };

export function hubSessionInitialStateV1(connectionId: string): HubSessionStateV1 {
  return { phase: "signed-out", connectionId, userId: null, expiresAtMs: null, role: null, error: null, retryAfterSeconds: null, offline: false };
}

/** 🧮️ Pure `state × event -> state`. A `connectivity` event never changes `phase`; a `failed` event
 * never clears a live session (a refresh failure while signed in is reported, not a sign-out) —
 * only `expired`/`signed-out` leave the signed-in phase. */
export function reduceHubSessionV1(state: HubSessionStateV1, event: HubSessionEventV1): HubSessionStateV1 {
  switch (event.kind) {
    case "select-connection":
      return event.connectionId === state.connectionId ? state : { ...hubSessionInitialStateV1(event.connectionId), offline: state.offline };
    case "submit":
      return { ...state, phase: "signing-in", error: null, retryAfterSeconds: null };
    case "minted":
      return { ...state, phase: "signed-in", userId: event.userId, expiresAtMs: event.expiresAtMs, error: null, retryAfterSeconds: null };
    case "failed":
      return state.phase === "signed-in"
        ? { ...state, error: event.code, retryAfterSeconds: event.retryAfterSeconds }
        : { ...state, phase: state.phase === "expired" ? "expired" : "signed-out", error: event.code, retryAfterSeconds: event.retryAfterSeconds };
    case "expired":
      return { ...state, phase: "expired", expiresAtMs: null, error: null, retryAfterSeconds: null };
    case "sign-out":
      return { ...state, phase: "signing-out", error: null, retryAfterSeconds: null };
    case "signed-out":
      return { ...hubSessionInitialStateV1(state.connectionId), offline: state.offline };
    case "connectivity":
      return event.offline === state.offline ? state : { ...state, offline: event.offline };
  }
}

/** 🏠️ Whether the app may keep editing locally. Always true — the predicate exists so the shell and
 * the tests state the law once instead of re-deriving it: no hub phase ever disables local work. */
export function hubSessionAllowsLocalWorkV1(_state: HubSessionStateV1): boolean {
  return true;
}

/** ♻️ Whether the human must be asked for credentials again before hub work resumes. */
export function hubSessionNeedsReauthenticationV1(state: HubSessionStateV1, nowMs: number): boolean {
  if (state.phase === "expired") return true;
  if (state.phase !== "signed-in") return false;
  return state.expiresAtMs !== null && state.expiresAtMs <= nowMs;
}
//#endregion 🔖️SessionState

//#region 🔖️Transport
/** 📨️ One hub answer, reduced to exactly what this module reads. Structural, so a test double is a
 * plain object and this file never names `Response`. */
export interface HubSignInResponseV1 {
  readonly status: number;
  readonly body: string;
  readonly retryAfterHeader: string | null;
}

/** 🔌️ The three calls the sign-in flow makes. `mint` posts the credential, `read` is
 * `GET /auth/sessions/me` (the re-auth probe), `end` is `DELETE /auth/sessions/me`. */
export interface HubSignInTransportV1 {
  mint(origin: string, body: string, signal: AbortSignal): Promise<HubSignInResponseV1>;
  read(origin: string, signal: AbortSignal): Promise<HubSignInResponseV1>;
  end(origin: string, signal: AbortSignal): Promise<HubSignInResponseV1>;
}

/** ✅️ One completed mint: the minted session plus the hub it belongs to. */
export type HubSignInOutcomeV1 =
  | { readonly kind: "minted"; readonly result: HubSessionMintResultV1 }
  | { readonly kind: "failed"; readonly code: HubSignInErrorCodeV1; readonly retryAfterSeconds: number | null };

/** 🔐️ Runs exactly one sign-in attempt and classifies every ending. Cancellation via `signal` is a
 * first-class outcome (`cancelled`), not an exception, so the caller's reducer stays total. A thrown
 * transport (DNS, TLS, timeout, offline) becomes `unreachable` — the app stays usable locally. */
export async function runHubSignInV1(
  transport: HubSignInTransportV1,
  origin: string,
  credential: HubSignInCredentialV1,
  signal: AbortSignal,
): Promise<HubSignInOutcomeV1> {
  if (signal.aborted) return { kind: "failed", code: "cancelled", retryAfterSeconds: null };
  let body: string;
  try {
    body = hubSessionMintRequestJsonV1(credential);
  } catch {
    return { kind: "failed", code: "malformed-request", retryAfterSeconds: null };
  }
  let response: HubSignInResponseV1;
  try {
    response = await transport.mint(origin, body, signal);
  } catch {
    return { kind: "failed", code: signal.aborted ? "cancelled" : "unreachable", retryAfterSeconds: null };
  }
  if (signal.aborted) return { kind: "failed", code: "cancelled", retryAfterSeconds: null };
  if (response.status !== 200) {
    const code = hubSignInErrorFromStatusV1(response.status);
    return { kind: "failed", code, retryAfterSeconds: code === "rate-limited" ? (hubRetryAfterSecondsV1(response.retryAfterHeader) ?? hubAuthErrorRetryAfterSecondsV1(response.body)) : null };
  }
  try {
    return { kind: "minted", result: parseHubSessionMintResultV1(response.body) };
  } catch {
    return { kind: "failed", code: "invalid-response", retryAfterSeconds: null };
  }
}

/** 🪪️ The exact `GET /auth/sessions/me` answer this module reads back. `expiresAtMs` is the whole
 * reason the call exists: AU1 §1.1 deliberately keeps the expiry out of the mint response, so the
 * deadline the shell re-authenticates against can only come from here. */
export interface HubSessionAuthorityV1 {
  readonly userId: string;
  readonly email: string;
  readonly displayName: string;
  readonly expiresAtMs: number;
  readonly sessionKind: "external" | "development-local";
  readonly authorizationGeneration: number;
}

export type HubSessionAuthorityOutcomeV1 =
  | { readonly kind: "authority"; readonly authority: HubSessionAuthorityV1 }
  | { readonly kind: "expired" }
  | { readonly kind: "failed"; readonly code: HubSignInErrorCodeV1 };

/** 📥️ Decodes exactly `DirectorySessionAuthorityV1` (`📇️directory/🧬️schema/🪪️session-authority-v1`).
 * The hub's own `validate()` refuses an unbounded or zeroed record before it is served; this mirrors
 * the fields the shell actually reads and refuses anything else, so a proxy's HTML can never be
 * installed as an identity. */
export function parseHubSessionAuthorityV1(source: string): HubSessionAuthorityV1 {
  if (new TextEncoder().encode(source).byteLength > HUB_SESSION_AUTHORITY_MAX_BYTES) throw new Error("hub.sign-in.response-too-large");
  const value: unknown = JSON.parse(source);
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("hub.sign-in.invalid-response");
  const object = value as Record<string, unknown>;
  if (object.schema !== HUB_SESSION_AUTHORITY_SCHEMA_V1) throw new Error("hub.sign-in.invalid-response");
  const { userId, email, displayName, expiresAt, sessionKind, authorizationGeneration } = object;
  if (typeof userId !== "string" || userId.length === 0 || userId.length > 256 || /\p{Cc}/u.test(userId)) throw new Error("hub.sign-in.invalid-response");
  if (typeof email !== "string" || email.length === 0 || email.length > 320 || /\p{Cc}/u.test(email)) throw new Error("hub.sign-in.invalid-response");
  if (typeof displayName !== "string" || displayName.length === 0 || displayName.length > 128 || /\p{Cc}/u.test(displayName)) throw new Error("hub.sign-in.invalid-response");
  if (typeof expiresAt !== "number" || !Number.isSafeInteger(expiresAt) || expiresAt < 1) throw new Error("hub.sign-in.invalid-response");
  if (sessionKind !== "external" && sessionKind !== "development-local") throw new Error("hub.sign-in.invalid-response");
  if (typeof authorizationGeneration !== "number" || !Number.isSafeInteger(authorizationGeneration) || authorizationGeneration < 1) throw new Error("hub.sign-in.invalid-response");
  return { userId, email, displayName, expiresAtMs: expiresAt, sessionKind, authorizationGeneration };
}

/** 🪪️ Reads the session's own authority record. A `401` is the hub saying this capability is gone
 * (expired, revoked, or its generation bumped by a credential change) — the one honest trigger for
 * re-authentication, and the reason the shell never has to guess from a local clock alone. */
export async function runHubSessionAuthorityV1(transport: HubSignInTransportV1, origin: string, signal: AbortSignal): Promise<HubSessionAuthorityOutcomeV1> {
  if (signal.aborted) return { kind: "failed", code: "cancelled" };
  let response: HubSignInResponseV1;
  try {
    response = await transport.read(origin, signal);
  } catch {
    return { kind: "failed", code: signal.aborted ? "cancelled" : "unreachable" };
  }
  if (signal.aborted) return { kind: "failed", code: "cancelled" };
  if (response.status === 401) return { kind: "expired" };
  if (response.status !== 200) return { kind: "failed", code: hubSignInErrorFromStatusV1(response.status) };
  try {
    return { kind: "authority", authority: parseHubSessionAuthorityV1(response.body) };
  } catch {
    return { kind: "failed", code: "invalid-response" };
  }
}

/** 🚪️ Ends the current session. A refusal is reported but never keeps the human signed in locally —
 * the shell drops its in-memory session either way, so an unreachable hub cannot trap a session. */
export async function runHubSignOutV1(transport: HubSignInTransportV1, origin: string, signal: AbortSignal): Promise<boolean> {
  try {
    const response = await transport.end(origin, signal);
    return response.status === 204 || response.status === 200;
  } catch {
    return false;
  }
}
//#endregion 🔖️Transport
