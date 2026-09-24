import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, test } from "vitest";
import { hubConnectionSummaryV1 } from "../../🧱️elements/🔄️ShellSync/🟦️.tsx";
import { hubConnectionOperationOwnerCurrentV1 } from "../../🧱️elements/🔗️HubConnection/🟦️.tsx";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🔗️hub-projection", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🔗️hub-projection", "🔣️.json"), "utf8"));

type Remote = { kind: "detached" | "connecting" } | { kind: "live"; peerCount: number } | { kind: "backoff"; retryInMs: number };
type Projection = { authority: { kind: "noVerifiedSession" } | { kind: "verifiedSession"; authorizationGeneration: number }; documents: Array<{ documentKey: string; remote: Remote }> };
type Summary = { state: string; peerCount: number; documentCount: number };

const independentSummary = (projection: Projection): Summary => {
  if (projection.authority.kind === "noVerifiedSession") return { state: "signedOut", peerCount: 0, documentCount: projection.documents.length };
  const peerCount = projection.documents.reduce((best, document) => document.remote.kind === "live" ? Math.max(best, document.remote.peerCount) : best, 0);
  if (projection.documents.some((document) => document.remote.kind === "live")) return { state: "live", peerCount, documentCount: projection.documents.length };
  if (projection.documents.some((document) => document.remote.kind === "connecting")) return { state: "connecting", peerCount: 0, documentCount: projection.documents.length };
  if (projection.documents.some((document) => document.remote.kind === "backoff")) return { state: "reconnecting", peerCount: 0, documentCount: projection.documents.length };
  return { state: "offline", peerCount: 0, documentCount: projection.documents.length };
};

describe("🔗️ target-neutral Hub projection", () => {
  test("the fixture satisfies its shared schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("the independent fold matches every authority and multi-document vector", () => {
    for (const row of fixture.cases as Array<{ projection: Projection; expected: Summary }>) expect(independentSummary(row.projection)).toEqual(row.expected);
  });

  test("the neutral owner and latest-publication laws reject a retired connection generation", () => {
    const ownership = fixture.operationOwnership as { selectedGeneration: number; completions: Array<{ ownerGeneration: number; accepted: boolean }> };
    for (const completion of ownership.completions) {
      expect(completion.ownerGeneration === ownership.selectedGeneration).toBe(completion.accepted);
      expect(hubConnectionOperationOwnerCurrentV1({ generation: completion.ownerGeneration, connectionId: "hub-b", origin: "https://b.invalid" }, ownership.selectedGeneration, "hub-b")).toBe(completion.accepted);
    }
    const transport = fixture.transport as { publications: Array<"connecting" | "live" | "close">; expected: string; capacity: number };
    expect(transport.publications.reduce((_current, publication) => publication)).toBe(transport.expected);
    expect(transport.capacity).toBe(64);
  });

  test("React is the third-party renderer oracle for the same fold", () => {
    for (const row of fixture.cases as Array<{ projection: Projection; expected: Summary }>) {
      const statuses = row.projection.documents.map((document) => ({ persisted: true, pendingMutations: 0, remote: document.remote }));
      const session = row.projection.authority.kind === "verifiedSession" ? "signedIn" : "signedOut";
      expect(hubConnectionSummaryV1(statuses, session)).toEqual(row.expected);
    }
  });

  test("the React workspace and host publish the neutral opener and sections", () => {
    const workspace = readFileSync(join(engineRoot, "🧱️elements", "🔗️HubConnection", "🏛️workspace", "🟦️.tsx"), "utf8");
    const host = readFileSync(join(engineRoot, "🧱️elements", "🏛️ShellHost", "🟦️.tsx"), "utf8");
    const shellSync = readFileSync(join(engineRoot, "🧱️elements", "🔄️ShellSync", "🟦️.tsx"), "utf8");
    const signIn = readFileSync(join(engineRoot, "🧱️elements", "🔐️HubSignIn", "🟦️.tsx"), "utf8");
    const spaces = readFileSync(join(engineRoot, "🧱️elements", "🏘️SpaceBrowser", "🟦️.tsx"), "utf8");
    const labels = readFileSync(join(engineRoot, "🧱️elements", "🔗️HubConnection", "🟦️.tsx"), "utf8");
    expect(shellSync).toContain("framework.hub.signIn");
    expect(host).toContain('const SHELL_HUB_ROUTE = "/hub";');
    expect(host).toContain("navigateHistory(SHELL_HUB_ROUTE)");
    expect(workspace).toContain("HubSignInPane");
    expect(workspace).toContain("SpaceBrowser");
    for (const id of ["os.hub.signIn.addHub", "os.hub.signIn.hubAddress", "os.hub.signIn.email", "os.hub.signIn.password"]) expect(signIn).toContain(id);
    for (const id of ["os.hub.spaces.refresh", "os.hub.spaces.search", "os.hub.spaces.members", "os.hub.invite.redeemField", "os.hub.invite.redeem"]) expect(spaces).toContain(id);
    for (const text of ["Add another hub", "Refresh spaces", "Find a space", "Members", "Invitation link or code", "Join space"]) expect(labels).toContain(text);
  });

  test("the browser host publishes every document status into the bounded wgpu projection", () => {
    const shell = readFileSync(join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    const host = readFileSync(join(engineRoot, "🧱️elements", "🏛️ShellHost", "🟦️.tsx"), "utf8");
    const transport = readFileSync(join(engineRoot, "🎯️targets", "🧊️wgpu", "🚚️browser-frame-transport", "🟦️.ts"), "utf8");
    const boot = readFileSync(join(engineRoot, "🎯️targets", "🧊️wgpu", "🚀️browser-boot", "🟦️.ts"), "utf8");
    expect(shell).toContain("pub hub_documents: BTreeMap<String, ShellHubRemoteV1>");
    expect(shell).toContain("verified_session_authority: Option<DirectorySessionAuthorityV1>");
    expect(host).toContain("semioWgpuHubProjection?.publishDocumentStatus(runtimeKey, status.remote)");
    expect(host).toContain("semioWgpuHubProjection?.publishDocumentStatus(runtimeKey, null)");
    expect(transport).toContain('kind: "hub-document-status"');
    expect(transport).toContain('kind: "hub-document-close"');
    expect(boot).toContain('WGPU_HUB_PROJECTION_GLOBAL = "semioWgpuHubProjection"');
  });
});
