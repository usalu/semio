import { expect, test } from "bun:test";
import { AUTH_CREDENTIAL_ROUTE, AUTH_SESSION_ME_ROUTE, AUTH_SESSION_MINT_ROUTE } from "../../../../🔐️auth/🧬️schema/🟦️.ts";
import { localRelayExecutionTargetAsset, localRelayInferencePath, localRelayInviteRedemptionPath, localRelaySpaceArtifactCreationPath, localRelayUpstreamPath } from "../../🟦️.ts";

/** 🧭 The relay's verdict for one browser-side path, always through the real `/_semio/hub` prefix. */
function relay(method: string, path: string): string | undefined {
  return localRelayUpstreamPath(method, new URL(`http://relay.invalid/_semio/hub${path}`));
}

test("the relay admits exactly the hub's own session-lifecycle routes, verb by verb", () => {
  for (const [method, route] of [
    ["POST", AUTH_SESSION_MINT_ROUTE],
    ["GET", AUTH_SESSION_ME_ROUTE],
    ["DELETE", AUTH_SESSION_ME_ROUTE],
    ["POST", AUTH_CREDENTIAL_ROUTE],
  ] as const) {
    expect(relay(method, route), `${method} ${route}`).toBe(route);
    expect(relay(method, `${route}?extra=1`), `${method} ${route} with a query`).toBeUndefined();
  }
  for (const [method, route] of [
    ["GET", AUTH_SESSION_MINT_ROUTE],
    ["DELETE", AUTH_SESSION_MINT_ROUTE],
    ["PUT", AUTH_SESSION_MINT_ROUTE],
    ["POST", AUTH_SESSION_ME_ROUTE],
    ["PUT", AUTH_SESSION_ME_ROUTE],
    ["GET", AUTH_CREDENTIAL_ROUTE],
    ["DELETE", AUTH_CREDENTIAL_ROUTE],
  ] as const) {
    expect(relay(method, route), `${method} ${route}`).toBeUndefined();
  }
});

test("a path that merely resembles an admitted auth route stays closed", () => {
  for (const path of [
    "/auth",
    "/auth/",
    "/auth/sessions/",
    "/auth/sessions/me/extra",
    "/auth/sessionsme",
    "/auth/sessions/mex",
    "/auth/credentials/reset",
    "/auth/credentials/",
    "/auth/sessions/../../admin/api/users",
    "/auth/sessions%2fme",
  ]) {
    for (const method of ["GET", "POST", "DELETE", "PUT"]) expect(relay(method, path), `${method} ${path}`).toBeUndefined();
  }
  expect(localRelayUpstreamPath("POST", new URL("http://relay.invalid/auth/sessions"))).toBeUndefined();
  expect(localRelayUpstreamPath("POST", new URL("http://relay.invalid/_semio/hubauth/sessions"))).toBeUndefined();
});

test("the relay admits exactly one bounded invite redemption", () => {
  const token = `invite.v1.${"a".repeat(32)}.${"b".repeat(64)}`;
  expect(relay("POST", `/directory/invites/${token}/redeem`)).toBe(`/directory/invites/${token}/redeem`);
  expect(relay("GET", `/directory/invites/${token}/redeem`)).toBeUndefined();
  expect(relay("POST", `/directory/invites/${token}/redeem?extra=1`)).toBeUndefined();
  expect(localRelayInviteRedemptionPath("POST", `/directory/invites/${token}/redeem`)).toBe(true);
  for (const hostile of ["", ".", "..", "%2e%2e", "a%2Fb", "a%20b", "a".repeat(513), ".leading", "-leading", "a b"]) {
    expect(localRelayInviteRedemptionPath("POST", `/directory/invites/${hostile}/redeem`), JSON.stringify(hostile)).toBe(false);
  }
  expect(localRelayInviteRedemptionPath("POST", `/directory/invites/${token}/redeem/extra`)).toBe(false);
  expect(localRelayInviteRedemptionPath("POST", `/directory/invites/${token}`)).toBe(false);
  expect(localRelayInviteRedemptionPath("GET", `/directory/invites/${token}/redeem`)).toBe(false);
});

test("the pre-existing admitted families are unchanged by the auth additions", () => {
  const scoped = "/directory/spaces/space%2Fa/documents/document%20b/socket-grants";
  expect(relay("POST", scoped)).toBe(scoped);
  expect(relay("GET", scoped)).toBeUndefined();
  expect(relay("GET", "/directory/spaces")).toBe("/directory/spaces");
  expect(relay("POST", "/directory/commands")).toBe("/directory/commands");
  expect(localRelayUpstreamPath("GET", new URL("http://relay.invalid/_semio/hub/directory/events?since=7"))).toBe("/directory/events?since=7");
  expect(localRelayExecutionTargetAsset("/spaces/space-a/documents/document-a/execution-target/browser-actor")).toBe("browser-actor");
  expect(localRelayExecutionTargetAsset("/spaces/../documents/document-a/execution-target/manifest")).toBeUndefined();
  expect(localRelaySpaceArtifactCreationPath("POST", `/spaces/space-a/artifact-creations/${"a".repeat(32)}/cancel`)).toBe(true);
  expect(localRelaySpaceArtifactCreationPath("POST", `/spaces/space-a/artifact-creations/${"0".repeat(32)}/cancel`)).toBe(false);
  expect(localRelayInferencePath("GET", `/spaces/space-a/documents/document-a/inference/gis-map/jobs/${"a".repeat(32)}/events?after=0`)).toBe(true);
  expect(localRelayInferencePath("GET", `/spaces/space-a/documents/document-a/inference/gis-map/jobs/${"a".repeat(32)}/events?after=999999999999`)).toBe(false);
});
