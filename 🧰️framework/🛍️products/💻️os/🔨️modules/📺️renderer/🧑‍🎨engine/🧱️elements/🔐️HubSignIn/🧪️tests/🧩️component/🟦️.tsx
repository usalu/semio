// #region 🧲️Header
/** @emoji 🔐️ Laws for the hub sign-in contract and pane (ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AU2). Two independent oracles per shape: an
 * Ajv compile of the owned fixture schema and a `fast-deep-equal` comparison against the fixture,
 * so the source tables cannot drift from the contract without a red test. The transport is a fake
 * implementing AU1 §1.1's real wire (status table, `retry-after`, `semio.hub.auth.error/v1` body,
 * `session.v1.<32hex>.<64hex>` token), never a mock of this module's own functions. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { act, cleanup, fireEvent, render, screen, waitFor } from "@semio-tech/ui-react/test";
import { setUiLocale } from "@semio-tech/ui-react";
import Ajv from "ajv";
import equal from "fast-deep-equal";
import { useState, type ReactElement } from "react";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import fixture from "../../../../../../📇️directory/🔐️sign-in/🔣️.json";
import fixtureSchema from "../../../../../../📇️directory/🔐️sign-in/🧬️.schema.json";
import {
  HUB_AUTH_ERROR_SCHEMA_V1,
  HUB_CONNECTION_BOOK_MAX_ENTRIES,
  HUB_SESSION_MINT_REQUEST_MAX_BYTES,
  HUB_SIGN_IN_REQUEST_SCHEMA_V1,
  HUB_SIGN_IN_TEXT_V1,
  LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1,
  hubAuthErrorRetryAfterSecondsV1,
  hubConnectionIdForOriginV1,
  hubRetryAfterSecondsV1,
  hubSessionInitialStateV1,
  hubSessionMintRequestJsonV1,
  hubSessionNeedsReauthenticationV1,
  parseHubSessionAuthorityV1,
  hubSessionAllowsLocalWorkV1,
  hubSignInErrorFromStatusV1,
  hubSignInTextV1,
  parseHubConnectionBookV1,
  parseHubOriginV1,
  parseHubSessionCapabilityV1,
  parseHubSessionMintResultV1,
  readHubConnectionBookV1,
  readHubSessionCapabilityV1,
  writeHubSessionCapabilityV1,
  HUB_SESSION_CAPABILITY_STORAGE_KEY_V1,
  reduceHubSessionV1,
  removeHubConnectionV1,
  runHubSignInV1,
  runHubSignOutV1,
  selectedHubConnectionV1,
  serializeHubConnectionBookV1,
  upsertHubConnectionV1,
  validHubSignInEmailV1,
  validHubSignInPasswordV1,
  writeHubConnectionBookV1,
  type HubConnectionStorageV1,
  type HubSessionStateV1,
  type HubSignInErrorCodeV1,
  type HubSignInResponseV1,
  type HubSignInTransportV1,
} from "../../../../../../📇️directory/🔐️sign-in/🟦️.ts";
import localSessionFixture from "../../../../../../📇️directory/🎫️local-session/🔣️.json";
import localSessionSchema from "../../../../../../📇️directory/🎫️local-session/🧬️.schema.json";
import { localHubSessionAnswerV1, parseLocalHubSessionAnswerV1, type LocalHubSessionV1 } from "../../../../../../📇️directory/🎫️local-session/🟦️.ts";
import { HubSignInPane, hubSignInFormOfferedV1, hubSignInSubmittableV1 } from "../../🟦️.tsx";
import { useHubConnection, type HubConnectionPortV1 } from "../../../🔗️HubConnection/🟦️.tsx";
// #endregion 🔌️Adapters

afterEach(cleanup);
beforeEach(async () => {
  await setUiLocale("en");
});

//#region 🧫️Doubles
const ORIGIN = "https://hub.example.invalid";
const TOKEN = "session.v1.0123456789abcdef0123456789abcdef.0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

function memoryStorage(seed?: string): HubConnectionStorageV1 & { readonly entries: Map<string, string> } {
  const entries = new Map<string, string>();
  if (seed !== undefined) entries.set("semio.os.hub-connection-book.v1", seed);
  return { entries, getItem: (key) => entries.get(key) ?? null, setItem: (key, value) => void entries.set(key, value) };
}

type FakeHubScript = {
  readonly mint: HubSignInResponseV1 | (() => Promise<HubSignInResponseV1>);
  readonly end?: HubSignInResponseV1;
  /** 🪪️ `GET /auth/sessions/me`. A hub that has just minted a session answers it — the shell reads
   * the deadline from here because the mint body deliberately carries none (AU1 §1.1), so a fake
   * that only answers `401` is a hub that revoked the session it just issued. */
  readonly read?: HubSignInResponseV1;
};

/** 🪪️ The exact `DirectorySessionAuthorityV1` a hub serves for the minted fixture session. */
const AUTHORITY_EXPIRES_AT_MS = 4_102_444_800_000;

function authorityBody(userId = "usr_ada"): string {
  return JSON.stringify({
    schema: "semio.directory.session-authority.v1",
    sessionBindingSha256: "c".repeat(64),
    authorizationGeneration: 1,
    userId,
    email: "ada@example.invalid",
    displayName: "Ada",
    expiresAt: AUTHORITY_EXPIRES_AT_MS,
    sessionKind: "external",
  });
}

function fakeTransport(script: FakeHubScript): HubSignInTransportV1 & { readonly bodies: string[] } {
  const bodies: string[] = [];
  return {
    bodies,
    mint: async (_origin, body) => {
      bodies.push(body);
      return typeof script.mint === "function" ? script.mint() : script.mint;
    },
    read: async () => script.read ?? { status: 200, body: authorityBody(), retryAfterHeader: null },
    end: async () => script.end ?? { status: 204, body: "", retryAfterHeader: null },
  };
}

function minted(): HubSignInResponseV1 {
  return { status: 200, body: JSON.stringify({ token: TOKEN, user_id: "usr_ada" }), retryAfterHeader: null };
}

function fakePort(transport: HubSignInTransportV1, storage: HubConnectionStorageV1 | null): HubConnectionPortV1 {
  return {
    signIn: transport,
    storage,
    bootstrapOrigin: "http://127.0.0.1:7777",
    deviceInstanceId: "device-au2",
    clientClass: "browser",
    listSpaces: async () => [],
    readSpaceMembers: async () => [],
    submitCommand: async () => {
      throw new Error("unused");
    },
    redeemInvite: async () => ({ status: 200 }),
    listAgentDelegations: async () => [],
    createAgentDelegation: async () => {
      throw new Error("unused");
    },
    revokeAgentDelegation: async () => undefined,
  };
}

function signedIn(): HubSessionStateV1 {
  return { ...hubSessionInitialStateV1("local-bootstrap"), phase: "signed-in", userId: "usr_ada" };
}

function book() {
  return parseHubConnectionBookV1(null, "http://127.0.0.1:7777");
}
//#endregion 🧫️Doubles

//#region 🧬️Contract
describe("hub sign-in contract", () => {
  it("validates its own fixture with an independent schema compiler and mirrors every text pair", () => {
    expect(new Ajv({ strict: true, allErrors: true }).compile(fixtureSchema)(fixture)).toBe(true);
    expect(equal(HUB_SIGN_IN_TEXT_V1, fixture.presentation.text)).toBe(true);
    for (const locale of fixture.presentation.locales) expect(equal(hubSignInTextV1(locale), fixture.presentation.text[locale as "en" | "de"])).toBe(true);
    expect(() => hubSignInTextV1("fr")).toThrow("hub.sign-in.locale-unsupported");
    expect(Object.keys(HUB_SIGN_IN_TEXT_V1.en).sort()).toStrictEqual(Object.keys(HUB_SIGN_IN_TEXT_V1.de).sort());
    for (const locale of ["en", "de"] as const) for (const value of Object.values(HUB_SIGN_IN_TEXT_V1[locale])) expect(value.length).toBeGreaterThan(0);
  });

  it("classifies every hub status in the fixture's table", () => {
    for (const row of fixture.statusCodes) {
      if (row.status === 200) continue;
      expect(hubSignInErrorFromStatusV1(row.status)).toBe(row.code as HubSignInErrorCodeV1);
    }
  });

  it("reads retry-after deltas and refuses HTTP-date and out-of-range forms", () => {
    for (const row of fixture.retryAfter) expect(hubRetryAfterSecondsV1(row.header)).toBe(row.seconds);
    expect(hubRetryAfterSecondsV1(null)).toBeNull();
    expect(hubAuthErrorRetryAfterSecondsV1(JSON.stringify({ schema: HUB_AUTH_ERROR_SCHEMA_V1, error: "rate-limited", retryAfterMs: 6000 }))).toBe(6);
    expect(hubAuthErrorRetryAfterSecondsV1(JSON.stringify({ schema: "other", retryAfterMs: 6000 }))).toBeNull();
    expect(hubAuthErrorRetryAfterSecondsV1("not json")).toBeNull();
  });

  it("normalizes typed hub addresses and refuses anything carrying a path, credential, query or fragment", () => {
    for (const row of fixture.origins) {
      if (row.origin === null) expect(() => parseHubOriginV1(row.typed)).toThrow("hub.sign-in.invalid-origin");
      else expect(parseHubOriginV1(row.typed)).toBe(row.origin);
    }
  });

  it("accepts only AU1's exact session capability shape and a two-field snake_case body", () => {
    for (const row of fixture.tokens) {
      const body = JSON.stringify({ token: row.token, user_id: "usr_ada" });
      if (row.valid) expect(parseHubSessionMintResultV1(body)).toStrictEqual({ token: row.token, userId: "usr_ada" });
      else expect(() => parseHubSessionMintResultV1(body)).toThrow("hub.sign-in.invalid-response");
    }
    expect(() => parseHubSessionMintResultV1(JSON.stringify({ token: TOKEN, user_id: "usr_ada", extra: 1 }))).toThrow("hub.sign-in.invalid-response");
    expect(() => parseHubSessionMintResultV1(JSON.stringify({ token: TOKEN }))).toThrow("hub.sign-in.invalid-response");
  });

  it("admits credentials against AU1's bounds before spending a rate-limit token", () => {
    for (const row of fixture.credentials) expect(validHubSignInEmailV1(row.email) && validHubSignInPasswordV1(row.password)).toBe(row.valid);
    expect(validHubSignInPasswordV1("a".repeat(257))).toBe(false);
    expect(validHubSignInEmailV1(`${"a".repeat(251)}@b.c`)).toBe(false);
    expect(validHubSignInEmailV1(`${"a".repeat(250)}@b.c`)).toBe(true);
  });

  it("seals the mint body in the declared field order, lowercases the email and stays under the body limit", () => {
    const body = hubSessionMintRequestJsonV1({ email: "Ada@Example.ORG", password: "correct horse", deviceInstanceId: "device-au2", clientClass: "browser" });
    expect(Object.keys(JSON.parse(body) as Record<string, unknown>)).toStrictEqual(fixture.requestFieldOrder);
    expect((JSON.parse(body) as { schema: string }).schema).toBe(HUB_SIGN_IN_REQUEST_SCHEMA_V1);
    expect((JSON.parse(body) as { email: string }).email).toBe("ada@example.org");
    expect(new TextEncoder().encode(body).byteLength).toBeLessThanOrEqual(HUB_SESSION_MINT_REQUEST_MAX_BYTES);
    expect(() => hubSessionMintRequestJsonV1({ email: "ada@example.org", password: "correct horse", deviceInstanceId: "bad device", clientClass: "browser" })).toThrow("hub.sign-in.invalid-device");
    expect(() => hubSessionMintRequestJsonV1({ email: "ada@example.org", password: "x".repeat(900), deviceInstanceId: "device-au2", clientClass: "browser" })).toThrow();
  });
});
//#endregion 🧬️Contract

//#region 📕️Book
describe("local-only hub connection book", () => {
  it("persists hub identity only and never a capability", () => {
    const storage = memoryStorage();
    const next = upsertHubConnectionV1(book(), { id: hubConnectionIdForOriginV1(ORIGIN), label: "Studio hub", origin: ORIGIN, kind: "remote", lastUserId: "usr_ada" });
    writeHubConnectionBookV1(storage, next);
    const written = storage.entries.get("semio.os.hub-connection-book.v1") ?? "";
    const parsed = JSON.parse(written) as { connections: Record<string, unknown>[] };
    expect(parsed.connections).toHaveLength(1);
    expect(Object.keys(parsed.connections[0]!).sort()).toStrictEqual(fixture.persistedFields);
    for (const forbidden of fixture.neverPersisted) expect(written.includes(forbidden)).toBe(false);
    expect(written.includes(TOKEN)).toBe(false);
    expect(serializeHubConnectionBookV1(parseHubConnectionBookV1(written, "http://127.0.0.1:7777"))).toBe(written);
  });

  it("repairs a corrupt, tampered or oversized book instead of blocking the app", () => {
    for (const source of ["", "null", "[]", '{"schema":"other"}', '{"schema":"semio.os.hub-connection-book.v1","connections":"x","selectedId":"a"}']) {
      const repaired = parseHubConnectionBookV1(source, "http://127.0.0.1:7777");
      expect(repaired.connections).toHaveLength(1);
      expect(repaired.selectedId).toBe(LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1);
    }
    const flooded = Array.from({ length: 40 }, (_, index) => ({ id: `remote:https://h${index}.invalid`, kind: "remote", label: `h${index}`, lastUserId: null, origin: `https://h${index}.invalid` }));
    const capped = parseHubConnectionBookV1(JSON.stringify({ schema: "semio.os.hub-connection-book.v1", connections: flooded, selectedId: "remote:https://h0.invalid" }), "http://127.0.0.1:7777");
    expect(capped.connections).toHaveLength(HUB_CONNECTION_BOOK_MAX_ENTRIES);
    const withPath = parseHubConnectionBookV1(JSON.stringify({ schema: "semio.os.hub-connection-book.v1", connections: [{ id: "remote:x", kind: "remote", label: "x", lastUserId: null, origin: "https://h.invalid/path" }], selectedId: "remote:x" }), "http://127.0.0.1:7777");
    expect(withPath.connections).toHaveLength(1);
  });

  it("remembers a hub session for THIS browsing context and refuses one minted by another hub", () => {
    const storage = memoryStorage();
    expect(readHubSessionCapabilityV1(storage, ORIGIN)).toBe(null);
    writeHubSessionCapabilityV1(storage, { origin: ORIGIN, token: TOKEN, userId: "usr_ada" });
    const restored = readHubSessionCapabilityV1(storage, ORIGIN);
    expect(restored).toStrictEqual({ origin: ORIGIN, token: TOKEN, userId: "usr_ada" });
    expect(readHubSessionCapabilityV1(storage, "https://other.invalid")).toBe(null);
    const book = storage.entries.get("semio.os.hub-connection-book.v1") ?? "";
    expect(book.includes(TOKEN)).toBe(false);
    writeHubSessionCapabilityV1(storage, null);
    expect(readHubSessionCapabilityV1(storage, ORIGIN)).toBe(null);
    expect(storage.entries.has(HUB_SESSION_CAPABILITY_STORAGE_KEY_V1)).toBe(true);
  });

  it("refuses every remembered record that is not exactly one capability for this origin", () => {
    const hostile = [
      null,
      "",
      "not json",
      "[]",
      JSON.stringify({ schema: "other", origin: ORIGIN, token: TOKEN, userId: "usr_ada" }),
      JSON.stringify({ schema: "semio.os.hub-session-capability.v1", origin: ORIGIN, token: TOKEN }),
      JSON.stringify({ schema: "semio.os.hub-session-capability.v1", origin: ORIGIN, token: TOKEN, userId: "usr_ada", extra: 1 }),
      JSON.stringify({ schema: "semio.os.hub-session-capability.v1", origin: ORIGIN, token: "session.v1.short", userId: "usr_ada" }),
      JSON.stringify({ schema: "semio.os.hub-session-capability.v1", origin: ORIGIN, token: `Bearer ${TOKEN}`, userId: "usr_ada" }),
      JSON.stringify({ schema: "semio.os.hub-session-capability.v1", origin: "https://elsewhere.invalid", token: TOKEN, userId: "usr_ada" }),
      JSON.stringify({ schema: "semio.os.hub-session-capability.v1", origin: ORIGIN, token: TOKEN, userId: "" }),
      "<!doctype html><title>proxy sign-in</title>",
    ];
    for (const source of hostile) expect(parseHubSessionCapabilityV1(source, ORIGIN)).toBe(null);
    expect(parseHubSessionCapabilityV1(JSON.stringify({ schema: "semio.os.hub-session-capability.v1", origin: ORIGIN, token: TOKEN, userId: "usr_ada" }), ORIGIN)).toStrictEqual({ origin: ORIGIN, token: TOKEN, userId: "usr_ada" });
  });

  it("treats a blocked capability store as no session rather than a boot failure", () => {
    const throwing: HubConnectionStorageV1 = {
      getItem: () => {
        throw new Error("blocked");
      },
      setItem: () => {
        throw new Error("blocked");
      },
    };
    expect(readHubSessionCapabilityV1(throwing, ORIGIN)).toBe(null);
    expect(() => writeHubSessionCapabilityV1(throwing, { origin: ORIGIN, token: TOKEN, userId: "usr_ada" })).not.toThrow();
  });

  it("survives a throwing storage and never removes the bootstrap entry", () => {
    const throwing: HubConnectionStorageV1 = {
      getItem: () => {
        throw new Error("blocked");
      },
      setItem: () => {
        throw new Error("blocked");
      },
    };
    const recovered = readHubConnectionBookV1(throwing, "http://127.0.0.1:7777");
    expect(recovered.connections).toHaveLength(1);
    expect(() => writeHubConnectionBookV1(throwing, recovered)).not.toThrow();
    const kept = removeHubConnectionV1(recovered, LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1);
    expect(selectedHubConnectionV1(kept).kind).toBe("local-bootstrap");
  });
});
//#endregion 📕️Book

//#region 🧮️Reducer
describe("hub session reducer", () => {
  it("never signs a live session out because the network dropped, and always allows local work", () => {
    const live = signedIn();
    const offline = reduceHubSessionV1(live, { kind: "connectivity", offline: true });
    expect(offline.phase).toBe("signed-in");
    expect(offline.offline).toBe(true);
    const refused = reduceHubSessionV1(offline, { kind: "failed", code: "unreachable", retryAfterSeconds: null });
    expect(refused.phase).toBe("signed-in");
    expect(refused.userId).toBe("usr_ada");
    expect(hubSessionAllowsLocalWorkV1(refused)).toBe(true);
  });

  it("drives submit → minted → expired → re-auth and clears identity on sign-out", () => {
    let state = hubSessionInitialStateV1("local-bootstrap");
    state = reduceHubSessionV1(state, { kind: "submit" });
    expect(state.phase).toBe("signing-in");
    state = reduceHubSessionV1(state, { kind: "minted", userId: "usr_ada", expiresAtMs: 1_000 });
    expect(hubSessionNeedsReauthenticationV1(state, 500)).toBe(false);
    expect(hubSessionNeedsReauthenticationV1(state, 2_000)).toBe(true);
    state = reduceHubSessionV1(state, { kind: "expired" });
    expect(state.phase).toBe("expired");
    expect(hubSessionNeedsReauthenticationV1(state, 0)).toBe(true);
    state = reduceHubSessionV1(state, { kind: "failed", code: "invalid-credentials", retryAfterSeconds: null });
    expect(state.phase).toBe("expired");
    state = reduceHubSessionV1(state, { kind: "signed-out" });
    expect(state.userId).toBeNull();
    expect(state.phase).toBe("signed-out");
  });

  it("resets identity when the chosen hub changes but keeps the observed connectivity", () => {
    const live = reduceHubSessionV1(signedIn(), { kind: "connectivity", offline: true });
    const switched = reduceHubSessionV1(live, { kind: "select-connection", connectionId: "remote:https://other.invalid" });
    expect(switched.userId).toBeNull();
    expect(switched.connectionId).toBe("remote:https://other.invalid");
    expect(switched.offline).toBe(true);
    expect(reduceHubSessionV1(switched, { kind: "select-connection", connectionId: "remote:https://other.invalid" })).toBe(switched);
  });
});
//#endregion 🧮️Reducer

//#region 🚚️Transport
describe("sign-in against a fake hub speaking the real contract", () => {
  it("mints a session on 200 and posts exactly one canonical body", async () => {
    const transport = fakeTransport({ mint: minted() });
    const outcome = await runHubSignInV1(transport, ORIGIN, { email: "ada@example.org", password: "correct horse", deviceInstanceId: "device-au2", clientClass: "browser" }, new AbortController().signal);
    expect(outcome).toStrictEqual({ kind: "minted", result: { token: TOKEN, userId: "usr_ada" } });
    expect(transport.bodies).toHaveLength(1);
  });

  it("maps 401, 403, 400 and 503 onto their own human-facing classes", async () => {
    for (const [status, code] of [[401, "invalid-credentials"], [403, "password-sign-in-disabled"], [400, "malformed-request"], [503, "unreachable"]] as const) {
      const transport = fakeTransport({ mint: { status, body: JSON.stringify({ schema: HUB_AUTH_ERROR_SCHEMA_V1, error: "x" }), retryAfterHeader: null } });
      const outcome = await runHubSignInV1(transport, ORIGIN, { email: "ada@example.org", password: "correct horse", deviceInstanceId: "device-au2", clientClass: "browser" }, new AbortController().signal);
      expect(outcome).toStrictEqual({ kind: "failed", code, retryAfterSeconds: null });
    }
  });

  it("carries the rate-limit countdown from the header, and from the error body when the header is missing", async () => {
    const withHeader = fakeTransport({ mint: { status: 429, body: "", retryAfterHeader: "6" } });
    expect(await runHubSignInV1(withHeader, ORIGIN, { email: "ada@example.org", password: "correct horse", deviceInstanceId: "device-au2", clientClass: "browser" }, new AbortController().signal)).toStrictEqual({ kind: "failed", code: "rate-limited", retryAfterSeconds: 6 });
    const withBody = fakeTransport({ mint: { status: 429, body: JSON.stringify({ schema: HUB_AUTH_ERROR_SCHEMA_V1, error: "rate-limited", retryAfterMs: 12_000 }), retryAfterHeader: null } });
    expect(await runHubSignInV1(withBody, ORIGIN, { email: "ada@example.org", password: "correct horse", deviceInstanceId: "device-au2", clientClass: "browser" }, new AbortController().signal)).toStrictEqual({ kind: "failed", code: "rate-limited", retryAfterSeconds: 12 });
  });

  it("treats a thrown transport as unreachable, a garbled 200 as invalid-response and an abort as cancelled", async () => {
    const thrown: HubSignInTransportV1 = { mint: async () => Promise.reject(new Error("dns")), read: async () => ({ status: 401, body: "", retryAfterHeader: null }), end: async () => ({ status: 204, body: "", retryAfterHeader: null }) };
    expect(await runHubSignInV1(thrown, ORIGIN, { email: "ada@example.org", password: "correct horse", deviceInstanceId: "device-au2", clientClass: "browser" }, new AbortController().signal)).toStrictEqual({ kind: "failed", code: "unreachable", retryAfterSeconds: null });
    const garbled = fakeTransport({ mint: { status: 200, body: "<!doctype html>", retryAfterHeader: null } });
    expect(await runHubSignInV1(garbled, ORIGIN, { email: "ada@example.org", password: "correct horse", deviceInstanceId: "device-au2", clientClass: "browser" }, new AbortController().signal)).toStrictEqual({ kind: "failed", code: "invalid-response", retryAfterSeconds: null });
    const abort = new AbortController();
    abort.abort();
    expect(await runHubSignInV1(fakeTransport({ mint: minted() }), ORIGIN, { email: "ada@example.org", password: "correct horse", deviceInstanceId: "device-au2", clientClass: "browser" }, abort.signal)).toStrictEqual({ kind: "failed", code: "cancelled", retryAfterSeconds: null });
  });

  it("reports sign-out on 204 and reports failure without trapping the human on an unreachable hub", async () => {
    expect(await runHubSignOutV1(fakeTransport({ mint: minted() }), ORIGIN, new AbortController().signal)).toBe(true);
    expect(await runHubSignOutV1(fakeTransport({ mint: minted(), end: { status: 503, body: "", retryAfterHeader: null } }), ORIGIN, new AbortController().signal)).toBe(false);
  });
});
//#endregion 🚚️Transport

//#region 🖥️Pane
function PaneHarness({ session, onSignIn }: { readonly session: HubSessionStateV1; readonly onSignIn?: (credential: Readonly<{ email: string; password: string }>) => void }): ReactElement {
  const [current] = useState(session);
  return (
    <HubSignInPane
      book={book()}
      session={current}
      locale="en"
      onSelectConnection={() => undefined}
      onAddHub={() => null}
      onForgetHub={() => undefined}
      onSignIn={onSignIn ?? (() => undefined)}
      onCancel={() => undefined}
      onSignOut={() => undefined}
    />
  );
}

describe("hub sign-in pane", () => {
  it("offers a labelled, autofillable credential form and only submits a well-formed credential", () => {
    let submissions = 0;
    const view = render(<PaneHarness session={hubSessionInitialStateV1("local-bootstrap")} onSignIn={() => { submissions += 1; }} />);
    const email = view.container.querySelector<HTMLInputElement>('input[type="email"]')!;
    const password = view.container.querySelector<HTMLInputElement>('input[type="password"]')!;
    expect(email.getAttribute("autocomplete")).toBe("username");
    expect(password.getAttribute("autocomplete")).toBe("current-password");
    expect(view.container.querySelector(`label[for="${email.id}"]`)?.textContent).toBe("Email");
    expect(view.container.querySelector(`label[for="${password.id}"]`)?.textContent).toBe("Password");
    const submit = screen.getByRole("button", { name: "Sign in" }) as HTMLButtonElement;
    expect(submit.disabled).toBe(true);
    fireEvent.change(email, { target: { value: "ada@example.org" } });
    fireEvent.change(password, { target: { value: "short" } });
    expect((screen.getByRole("button", { name: "Sign in" }) as HTMLButtonElement).disabled).toBe(true);
    expect(password.getAttribute("aria-invalid")).toBe("true");
    fireEvent.change(password, { target: { value: "correct horse" } });
    fireEvent.click(screen.getByRole("button", { name: "Sign in" }));
    expect(submissions).toBe(1);
  });

  it("announces every denial in one alert region and keeps a rate-limited attempt from resubmitting", () => {
    const rateLimited: HubSessionStateV1 = { ...hubSessionInitialStateV1("local-bootstrap"), error: "rate-limited", retryAfterSeconds: 30 };
    const view = render(<PaneHarness session={rateLimited} />);
    const alert = screen.getByRole("alert");
    expect(alert.textContent).toBe("Too many sign-in attempts. Try again in 30 s.");
    expect(alert.getAttribute("data-semio-hub-error")).toBe("rate-limited");
    expect(hubSignInSubmittableV1(rateLimited, "ada@example.org", "correct horse")).toBe(false);
    view.unmount();
    const wrong: HubSessionStateV1 = { ...hubSessionInitialStateV1("local-bootstrap"), error: "invalid-credentials", retryAfterSeconds: null };
    render(<PaneHarness session={wrong} />);
    expect(screen.getByRole("alert").textContent).toBe(HUB_SIGN_IN_TEXT_V1.en.invalidCredentials);
  });

  it("removes the password form entirely when the hub refuses credential sign-in", () => {
    const disabled: HubSessionStateV1 = { ...hubSessionInitialStateV1("local-bootstrap"), error: "password-sign-in-disabled", retryAfterSeconds: null };
    expect(hubSignInFormOfferedV1(disabled)).toBe(false);
    const view = render(<PaneHarness session={disabled} />);
    expect(view.container.querySelector('input[type="password"]')).toBeNull();
    expect(screen.getByRole("alert").textContent).toBe(HUB_SIGN_IN_TEXT_V1.en.passwordDisabled);
  });

  it("shows a busy submit with a cancel beside it while an attempt is in flight", () => {
    const view = render(<PaneHarness session={{ ...hubSessionInitialStateV1("local-bootstrap"), phase: "signing-in" }} />);
    const submit = screen.getByRole("button", { name: "Signing in…" }) as HTMLButtonElement;
    expect(submit.getAttribute("aria-busy")).toBe("true");
    expect(submit.disabled).toBe(true);
    expect(screen.getByRole("button", { name: "Cancel sign-in" })).toBeTruthy();
    expect(view.container.querySelector<HTMLInputElement>('input[type="email"]')!.disabled).toBe(true);
  });

  it("keeps a visible local-only affordance in every phase, including offline and signed-in", () => {
    for (const session of [hubSessionInitialStateV1("local-bootstrap"), signedIn(), { ...signedIn(), offline: true }, { ...hubSessionInitialStateV1("local-bootstrap"), phase: "expired" as const }]) {
      const view = render(<PaneHarness session={session} />);
      const line = view.container.querySelector('[data-semio-hub-local-only="true"]');
      expect(line?.textContent?.includes("Working on this device only")).toBe(true);
      if (session.offline) expect(line?.textContent?.includes("keep working locally")).toBe(true);
      view.unmount();
    }
  });

  it("offers sign-out only while signed in and names the signed-in user", () => {
    const view = render(<PaneHarness session={signedIn()} />);
    expect(screen.getByText("Signed in as usr_ada")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Sign out" })).toBeTruthy();
    expect(view.container.querySelector('input[type="password"]')).toBeNull();
    view.unmount();
    render(<PaneHarness session={hubSessionInitialStateV1("local-bootstrap")} />);
    expect(screen.queryByText("Signed in as usr_ada")).toBeNull();
  });

  it("renders every label in German without any English leaking through", async () => {
    await setUiLocale("de");
    const view = render(
      <HubSignInPane
        book={book()}
        session={{ ...hubSessionInitialStateV1("local-bootstrap"), error: "invalid-credentials", retryAfterSeconds: null }}
        locale="de"
        onSelectConnection={() => undefined}
        onAddHub={() => null}
        onForgetHub={() => undefined}
        onSignIn={() => undefined}
        onCancel={() => undefined}
        onSignOut={() => undefined}
      />,
    );
    expect(screen.getByRole("button", { name: "Anmelden" })).toBeTruthy();
    expect(screen.getByRole("alert").textContent).toBe(HUB_SIGN_IN_TEXT_V1.de.invalidCredentials);
    expect(view.container.textContent?.includes("Sign in")).toBe(false);
    expect(view.container.textContent?.includes("Password")).toBe(false);
    await setUiLocale("en");
  });

  it("refuses a hub address with a path and reports it inline without submitting", () => {
    const view = render(
      <HubSignInPane
        book={book()}
        session={hubSessionInitialStateV1("local-bootstrap")}
        locale="en"
        onSelectConnection={() => undefined}
        onAddHub={(typed) => {
          try {
            parseHubOriginV1(typed);
            return null;
          } catch {
            return "invalid-origin";
          }
        }}
        onForgetHub={() => undefined}
        onSignIn={() => undefined}
        onCancel={() => undefined}
        onSignOut={() => undefined}
      />,
    );
    const address = view.container.querySelector<HTMLInputElement>('input[name="hubOrigin"]')!;
    fireEvent.change(address, { target: { value: "https://hub.example.org/spaces" } });
    fireEvent.click(screen.getByRole("button", { name: "Add hub" }));
    expect(address.getAttribute("aria-invalid")).toBe("true");
    expect(view.container.textContent?.includes(HUB_SIGN_IN_TEXT_V1.en.invalidOrigin)).toBe(true);
  });
});
//#endregion 🖥️Pane

//#region 🔗️Hook
function HookHarness({ port }: { readonly port: HubConnectionPortV1 }): ReactElement {
  const hub = useHubConnection(port);
  return (
    <div>
      <HubSignInPane
        book={hub.book}
        session={hub.session}
        locale="en"
        onSelectConnection={hub.selectConnection}
        onAddHub={hub.addRemoteHub}
        onForgetHub={hub.forgetHub}
        onSignIn={hub.signIn}
        onCancel={hub.cancelSignIn}
        onSignOut={hub.signOut}
      />
      <output data-testid="phase">{hub.session.phase}</output>
      <output data-testid="user">{hub.session.userId ?? ""}</output>
      <output data-testid="hub">{hub.connection.origin}</output>
      <output data-testid="expires">{hub.session.expiresAtMs === null ? "" : String(hub.session.expiresAtMs)}</output>
      <output data-testid="display">{hub.authority?.displayName ?? ""}</output>
    </div>
  );
}

// 🪪️ ticket 26/09/18 slice AU3 — AU2 §6 gap 4: `expiresAtMs` was always `null` after a mint, so
// `hubSessionNeedsReauthenticationV1` could only fire on an explicit `expired` event. The shell now
// reads `GET /auth/sessions/me` right after minting, and that read is also the re-auth trigger.
describe("session authority follow-up", () => {
  it("reads the session's own authority after a mint and records the deadline the mint body withholds", async () => {
    const port = fakePort(fakeTransport({ mint: minted() }), memoryStorage());
    const view = render(<HookHarness port={port} />);
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="email"]')!, { target: { value: "ada@example.org" } });
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="password"]')!, { target: { value: "correct horse" } });
    fireEvent.click(screen.getByRole("button", { name: "Sign in" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="expires"]')?.textContent).toBe(String(AUTHORITY_EXPIRES_AT_MS)));
    expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("signed-in");
    expect(view.container.querySelector('[data-testid="display"]')?.textContent).toBe("Ada");
  });

  it("treats a 401 from the authority read as the hub revoking the session, not as a signed-in session", async () => {
    const port = fakePort(fakeTransport({ mint: minted(), read: { status: 401, body: "", retryAfterHeader: null } }), memoryStorage());
    const view = render(<HookHarness port={port} />);
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="email"]')!, { target: { value: "ada@example.org" } });
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="password"]')!, { target: { value: "correct horse" } });
    fireEvent.click(screen.getByRole("button", { name: "Sign in" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("expired"));
    expect(view.container.querySelector('[data-testid="expires"]')?.textContent).toBe("");
    expect(hubSessionNeedsReauthenticationV1({ ...hubSessionInitialStateV1("local-bootstrap"), phase: "expired" }, Date.now())).toBe(true);
  });

  it("refuses an authority record that is not the hub's own closed shape", () => {
    const admitted = parseHubSessionAuthorityV1(authorityBody());
    expect(admitted.expiresAtMs).toBe(AUTHORITY_EXPIRES_AT_MS);
    expect(admitted.sessionKind).toBe("external");
    const base = JSON.parse(authorityBody()) as Record<string, unknown>;
    for (const hostile of [
      { ...base, schema: "semio.directory.session-authority.v2" },
      { ...base, expiresAt: 0 },
      { ...base, expiresAt: 1.5 },
      { ...base, expiresAt: "4102444800000" },
      { ...base, sessionKind: "root" },
      { ...base, authorizationGeneration: 0 },
      { ...base, userId: "" },
      { ...base, displayName: "a".repeat(129) },
    ]) {
      expect(() => parseHubSessionAuthorityV1(JSON.stringify(hostile)), JSON.stringify(hostile).slice(0, 80)).toThrow();
    }
    expect(() => parseHubSessionAuthorityV1("<!doctype html><title>proxy</title>")).toThrow();
  });
});

describe("useHubConnection sign-in lane", () => {
  it("signs in through the pane, records the user and remembers the hub without the token", async () => {
    const storage = memoryStorage();
    const port = fakePort(fakeTransport({ mint: minted() }), storage);
    const view = render(<HookHarness port={port} />);
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="email"]')!, { target: { value: "ada@example.org" } });
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="password"]')!, { target: { value: "correct horse" } });
    fireEvent.click(screen.getByRole("button", { name: "Sign in" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("signed-in"));
    expect(view.container.querySelector('[data-testid="user"]')?.textContent).toBe("usr_ada");
    expect((storage.entries.get("semio.os.hub-connection-book.v1") ?? "").includes(TOKEN)).toBe(false);
  });

  it("reports a rate-limited attempt in the pane and leaves the human signed out", async () => {
    const port = fakePort(fakeTransport({ mint: { status: 429, body: "", retryAfterHeader: "42" } }), memoryStorage());
    const view = render(<HookHarness port={port} />);
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="email"]')!, { target: { value: "ada@example.org" } });
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="password"]')!, { target: { value: "correct horse" } });
    fireEvent.click(screen.getByRole("button", { name: "Sign in" }));
    await waitFor(() => expect(screen.getByRole("alert").textContent).toBe("Too many sign-in attempts. Try again in 42 s."));
    expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("signed-out");
  });

  it("cancels a slow attempt without ever installing the session that lands late", async () => {
    let release: (value: HubSignInResponseV1) => void = () => undefined;
    const pending = new Promise<HubSignInResponseV1>((resolve) => {
      release = resolve;
    });
    const port = fakePort(fakeTransport({ mint: () => pending }), memoryStorage());
    const view = render(<HookHarness port={port} />);
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="email"]')!, { target: { value: "ada@example.org" } });
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="password"]')!, { target: { value: "correct horse" } });
    fireEvent.click(screen.getByRole("button", { name: "Sign in" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("signing-in"));
    fireEvent.click(screen.getByRole("button", { name: "Cancel sign-in" }));
    expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("signed-out");
    act(() => release(minted()));
    await waitFor(() => expect(view.container.querySelector('[data-testid="user"]')?.textContent).toBe(""));
  });

  it("adds a remote hub, switches to it, and forgets it back to the local bootstrap", async () => {
    const storage = memoryStorage();
    const port = fakePort(fakeTransport({ mint: minted() }), storage);
    const view = render(<HookHarness port={port} />);
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[name="hubOrigin"]')!, { target: { value: "hub.example.invalid" } });
    fireEvent.click(screen.getByRole("button", { name: "Add hub" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="hub"]')?.textContent).toBe(ORIGIN));
    fireEvent.click(screen.getByRole("button", { name: "Forget this hub" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="hub"]')?.textContent).toBe("http://127.0.0.1:7777"));
  });

  it("signs out and clears the identity even when the hub refuses the revocation", async () => {
    const port = fakePort(fakeTransport({ mint: minted(), end: { status: 503, body: "", retryAfterHeader: null } }), memoryStorage());
    const view = render(<HookHarness port={port} />);
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="email"]')!, { target: { value: "ada@example.org" } });
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="password"]')!, { target: { value: "correct horse" } });
    fireEvent.click(screen.getByRole("button", { name: "Sign in" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("signed-in"));
    fireEvent.click(screen.getByRole("button", { name: "Sign out" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("signed-out"));
    expect(view.container.querySelector('[data-testid="user"]')?.textContent).toBe("");
  });
});
//#endregion 🔗️Hook

//#region 🎫️LocalSession
/** 🎫️ LAW over `📇️directory/🎫️local-session/🔣️.json`: the dev serve's local-session endpoint answers a typed session
 * or the typed "not offered" (never a 404 — ticket 26/09/23 U5). The encoder yields exactly the fixture's answers, the
 * shell adopts exactly the fixture's sessions, and both third-party oracles agree — Ajv on which bodies are
 * well-formed answers, `fast-deep-equal` on every encoded answer. */
describe("development local-session answers", () => {
  it("encode and decode every fixture row, and agree with the Ajv and fast-deep-equal oracles", () => {
    const rows = localSessionFixture as unknown as {
      readonly encode: readonly { readonly id: string; readonly session: LocalHubSessionV1 | null; readonly answer: unknown }[];
      readonly decode: readonly { readonly id: string; readonly body: unknown; readonly shape: boolean; readonly expected: LocalHubSessionV1 | null }[];
    };
    const validate = new Ajv({ strict: true }).compile(localSessionSchema as object);
    expect(rows.encode.length).toBeGreaterThanOrEqual(2);
    for (const row of rows.encode) {
      expect(equal(localHubSessionAnswerV1(row.session), row.answer), row.id).toBe(true);
      expect(validate(row.answer), `${row.id}: the encoded answer is a schema answer`).toBe(true);
      expect(parseLocalHubSessionAnswerV1(row.answer), `${row.id}: round trip`).toEqual(row.session);
    }
    expect(rows.decode.length).toBeGreaterThanOrEqual(10);
    for (const row of rows.decode) {
      expect(parseLocalHubSessionAnswerV1(row.body), row.id).toEqual(row.expected);
      expect(validate(row.body), `${row.id}: Ajv shape`).toBe(row.shape);
      if (row.expected !== null) expect(row.shape, `${row.id}: only a well-formed answer is ever adopted`).toBe(true);
    }
  });
});
//#endregion 🎫️LocalSession
