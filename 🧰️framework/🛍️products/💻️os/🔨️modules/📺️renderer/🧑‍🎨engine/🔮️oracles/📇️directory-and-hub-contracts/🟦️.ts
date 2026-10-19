/** 📇️ The language-neutral Node oracles of the directory and hub contracts: the visible retained Home identity/ACK bridge, the os sign-in contract against the hub's own `hub.auth` schema, the invite transfer machine and the exact-scope presence projection.
 * @see ../../🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts — the verbs that run it */

import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "../../../../../../../📦️packages/🟦️typescript/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { readModuleTreeSource } from "../../../../../../../📦️packages/🟦️typescript/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { ownedExport } from "../🧬️owned-schema-exports/🟦️.ts";

/** 📇️ Proves the visible retained Home identity/ACK bridge and language-neutral boundaries. */
export function directoryHomeBootstrapOracle(repoRoot: string): number {
  const shellHostRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost");
  const contractRoot = join(shellHostRoot, "📇️directory-bootstrap");
  const fixture = JSON.parse(readFileSync(join(shellHostRoot, "🧪️tests/🧫️fixtures/📇️directory-bootstrap/🔣️.json"), "utf8")) as {
    receipt: Readonly<Record<string, unknown>>;
    identities: Readonly<Record<"a" | "b", Readonly<{ userId: string; displayName: string }>>>;
    hostile: readonly { readonly id: string; readonly patch: Readonly<Record<string, unknown>> }[];
    lifecycle: readonly { readonly id: string; readonly action: string; readonly expected: string }[];
    labels: Readonly<Record<"en" | "de", readonly string[]>>;
  };
  const receiptExport = ownedExport(repoRoot, "directory", "DirectoryProjectionReceiptV1");
  const identityExport = ownedExport(repoRoot, "directory", "DirectoryHomeIdentityV1");
  const actionExport = ownedExport(repoRoot, "directory", "DirectoryHomeBootstrapActionV1");
  const outcomeExport = ownedExport(repoRoot, "directory", "DirectoryHomeBootstrapOutcomeV1");
  const labelsExport = ownedExport(repoRoot, "directory", "DirectoryHomeBootstrapLabelsV1");
  assert(receiptExport(fixture.receipt), JSON.stringify(receiptExport.errors));
  for (const identity of Object.values(fixture.identities)) assert(identityExport(identity), JSON.stringify(identityExport.errors));
  for (const step of fixture.lifecycle) {
    assert(actionExport(step.action), JSON.stringify(actionExport.errors));
    assert(outcomeExport(step.expected), JSON.stringify(outcomeExport.errors));
  }
  assert(labelsExport(fixture.labels), JSON.stringify(labelsExport.errors));
  const receiptKeys = ["schema", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive", "receiptSha256"].sort();
  const receipt = (value: unknown): boolean => {
    if (!value || typeof value !== "object" || Array.isArray(value) || JSON.stringify(Object.keys(value).sort()) !== JSON.stringify(receiptKeys)) return false;
    const row = value as Record<string, unknown>;
    return (
      row.schema === "semio.space.home.directory-projection-receipt.v1" &&
      typeof row.sessionBindingSha256 === "string" &&
      /^[0-9a-f]{64}$/u.test(row.sessionBindingSha256) &&
      Number.isSafeInteger(row.authorizationGeneration) &&
      Number(row.authorizationGeneration) > 0 &&
      Number.isSafeInteger(row.throughSeqInclusive) &&
      Number(row.throughSeqInclusive) >= 0 &&
      typeof row.receiptSha256 === "string" &&
      /^[0-9a-f]{64}$/u.test(row.receiptSha256)
    );
  };
  assert(receipt(fixture.receipt));
  for (const row of fixture.hostile) assert.equal(receipt({ ...structuredClone(fixture.receipt), ...row.patch }), false, row.id);
  const owner = readFileSync(join(contractRoot, "🟦️.tsx"), "utf8");
  const shell = readModuleTreeSource(join(contractRoot, "../🟦️.tsx"));
  const runtime = readModuleTreeSource(join(contractRoot, "../../🔌️PluginRuntime/🟦️.tsx"));
  assert(owner.includes("await owner.plugin.handleAction") && owner.includes("parseDirectoryProjectionReceiptV1(await settle.terminalOutputOf(operation))"));
  assert(!owner.includes("parseDirectoryProjectionReceiptV1(response.output)") && !owner.includes("parseDirectoryProjectionReceiptV1(admission.output)"));
  assert(owner.includes("owner.plugin.subscribeOperationCompletions(owner.instanceId, receive)") && owner.includes("startedDirectoryOperationIdV1(admission.output)"));
  assert(owner.indexOf("subscribeOperationCompletions") < owner.indexOf("await owner.plugin.handleAction"), "the completion subscription must open before the dispatch it settles");
  assert(runtime.includes("function takeTypedOperationTerminalOutputV1") && runtime.includes("terminalOutput,") && runtime.includes("completionFanouts"));
  assert(runtime.includes("pendingTurnEffects.get(instanceId)") && runtime.includes("typedOperationTerminalOutputV1([...completionLeftover, ...carried])"));
  assert(runtime.includes('effect.tag === "publish-event"') && runtime.includes('decodeWirePack((effect.val as { readonly payload?: unknown }).payload, "typed-operation.terminal-output")'));
  assert(owner.indexOf("await owner.plugin.handleAction") < owner.indexOf('kind: "directory-bootstrap-ack"'));
  assert(owner.includes('sessionIdentity: { userId: input.identity.userId, displayName: input.identity.displayName }'));
  assert(owner.includes('sessionIdentity: { userId: owner.identity.userId, displayName: owner.identity.displayName }'));
  assert(!owner.includes('directoryActionInvocation(owner, "setClient"'));
  assert(owner.includes("ownsInstance") && owner.includes("if (owner.ownsInstance)") && owner.includes("await beforeAcknowledge?.(owner)"));
  assert(owner.includes('kind: "directory-bootstrap-reject"') && owner.includes('kind: "directory-bootstrap-close"'));
  assert(shell.includes('message.kind === "directory-event-page"') && shell.includes("directoryHomeOwnerRef"));
  assert(shell.includes("openDirectoryHomeOwnerV1") && !shell.includes('kind: "directory-open", baseUrl: resolved.hubBaseUrl'));
  assert(shell.includes("instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState }") && shell.includes("directoryHomeOpeningRef.current.catch"));
  assert(shell.includes("identity: { userId: identity.userId, displayName: identity.displayName }") && shell.includes("await refreshDirectoryHomeRef.current(active)"));
  assert(runtime.includes("directory projection receipt was exposed before typed-operation terminal publication"));
  assert(runtime.includes("terminalOutputs.length === 1") && runtime.includes("output = terminalOutputs[0]!.val"));
  assert.deepEqual(Object.keys(fixture.labels).sort(), ["de", "en"]);
  assert(fixture.labels.en.every((label: string) => label.length > 0) && fixture.labels.de.every((label: string) => label.length > 0));
  assert.deepEqual(Object.keys(fixture.identities).sort(), ["a", "b"]);
  assert(fixture.identities.a.userId !== fixture.identities.b.userId && Object.values(fixture.identities).every((identity) => identity.userId.length > 0 && identity.displayName.length > 0));
  return 33;
}
/** 🔐️ Proves the os sign-in contract still agrees with the hub's OWN `hub.auth` schema — the only
 * cross-implementation oracle for a wire neither side can validate alone. Compares the hub's
 * `$defs/CredentialSignInRequestV1`/`SessionMintResponseV1`/`AuthErrorV1`/`AuthErrorCodeV1` against
 * the os fixture (`📇️directory/🔐️sign-in/🔣️.json`) and against the Rust status table in
 * `🌎️hub/🔐️auth/🦀️.rs`, so an AU1-side change to the route breaks this gate rather than the app. */
export function hubAuthContractOracle(repoRoot: string): number {
  const hubSchema = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/🔐️auth/🧬️schema/🔣️.json"), "utf8")) as { $defs: Record<string, Record<string, unknown>> };
  const osFixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔐️sign-in/🔣️.json"), "utf8")) as {
    signOutPath: string;
    requestSchema: string;
    errorSchema: string;
    requestMaxBytes: number;
    requestFieldOrder: readonly string[];
    statusCodes: readonly { readonly status: number; readonly code: string }[];
    tokens: readonly { readonly token: string; readonly valid: boolean }[];
    credentials: readonly { readonly email: string; readonly password: string; readonly valid: boolean }[];
  };
  const request = hubSchema.$defs.CredentialSignInRequestV1!;
  const requestProperties = request.properties as Record<string, Record<string, unknown>>;
  assert.equal((requestProperties.schema as { const: string }).const, osFixture.requestSchema);
  assert.equal((hubSchema.$defs.AuthErrorV1!.properties as Record<string, { const?: string }>).schema!.const, osFixture.errorSchema);
  assert.deepEqual(request.required, osFixture.requestFieldOrder);
  const rust = readFileSync(join(repoRoot, "🌎️hub/🔐️auth/🦀️.rs"), "utf8");
  assert.match(rust, new RegExp(`SIGN_IN_REQUEST_MAX_BYTES: usize = ${osFixture.requestMaxBytes};`));
  assert.ok(rust.includes(`SESSION_SIGN_OUT_ROUTE: &str = "${osFixture.signOutPath}";`), "the os sign-out path is the hub's own sign-out command route");
  const validate = semioSchemaAjvV1({ strict: false, allErrors: true }).addSchema(hubSchema).getSchema(`${"https://json.schemas.assets.semio-tech.com/hub/auth/schema.json"}#/$defs/CredentialSignInRequestV1`);
  if (!validate) throw new Error("hub auth schema publishes no CredentialSignInRequestV1");
  for (const row of osFixture.credentials) {
    const body = { schema: osFixture.requestSchema, email: row.email.trim().toLowerCase(), password: row.password, deviceInstanceId: "device-au2", clientClass: "browser" };
    assert.equal(validate(body) === true, row.valid, `credential admission disagrees with the hub schema for ${row.email}`);
  }
  const token = semioSchemaAjvV1({ strict: false }).addSchema(hubSchema).getSchema(`${"https://json.schemas.assets.semio-tech.com/hub/auth/schema.json"}#/$defs/SessionCapabilityV1`);
  if (!token) throw new Error("hub auth schema publishes no SessionCapabilityV1");
  for (const row of osFixture.tokens) assert.equal(token(row.token) === true, row.valid, `token admission disagrees with the hub schema for ${row.token.slice(0, 24)}`);
  const codes = (hubSchema.$defs.AuthErrorCodeV1!.enum as readonly string[]).slice().sort();
  assert.deepEqual(codes, ["credential-sign-in-disabled", "directory-unavailable", "invalid-credentials", "malformed-request", "rate-limited"]);
  for (const [code, status] of [["MalformedRequest", 400], ["InvalidCredentials", 401], ["CredentialSignInDisabled", 403], ["RateLimited", 429], ["DirectoryUnavailable", 503]] as const) {
    assert.match(rust, new RegExp(`Self::${code} => ${status},`), `hub status for ${code} moved`);
    assert(osFixture.statusCodes.some((row) => row.status === status), `os fixture has no row for status ${status}`);
  }
  return osFixture.credentials.length + osFixture.tokens.length + codes.length;
}
/** 🎟️ Proves the language-neutral invite transfer machine and its typed browser/worker bridge. */
export function directoryInviteCapabilityOracle(repoRoot: string): number {
  const contractRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎟️invite-capability");
  const fixture = JSON.parse(readFileSync(join(contractRoot, "🔣️.json"), "utf8")) as {
    states: readonly string[];
    transitions: readonly { readonly from: string; readonly input: string; readonly to: string; readonly discloses: boolean; readonly erases: boolean }[];
    duplicateResult: string;
    shellRoute: Readonly<{ actionId: string; spaceId: string; operationEpoch: number; openingPhase: string; requestKind: string }>;
    disclosureAuthority: readonly string[];
    focusPhases: readonly string[];
    labels: Readonly<Record<"en" | "de", readonly string[]>>;
  };
  const transferExport = ownedExport(repoRoot, "directory", "InviteCapabilityTransferV1");
  assert(transferExport(fixture), JSON.stringify(transferExport.errors));
  assert.deepEqual(fixture.states, ["available", "copying", "failed", "copied", "closed"]);
  assert.deepEqual(
    fixture.transitions.filter((row) => row.discloses).map((row) => [row.from, row.input, row.to]),
    [
      ["available", "request", "copying"],
      ["failed", "request", "copying"],
    ],
  );
  assert(fixture.transitions.filter((row) => row.erases).every((row) => row.to === "copied" || row.to === "closed"));
  assert.equal(fixture.duplicateResult, "reject-without-disclosure");
  assert.deepEqual(fixture.shellRoute, { actionId: "os.directory.open-administration", spaceId: "space-admin-01", operationEpoch: 7, openingPhase: "loading", requestKind: "directory-administration-open" });
  assert.deepEqual(fixture.disclosureAuthority, ["author"]);
  assert.deepEqual(fixture.focusPhases, ["ready", "denied", "stale", "failed", "cancelled"]);
  assert.deepEqual(Object.keys(fixture.labels).sort(), ["de", "en"]);
  assert(Object.values(fixture.labels).every((labels) => labels.length === 3 && labels.every((label) => label.length > 0)));
  const worker = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"));
  const shell = readModuleTreeSource(join(contractRoot, "../🟦️.tsx"));
  const pane = readFileSync(join(contractRoot, "../../🛂️SpaceAdministration/🟦️.tsx"), "utf8");
  const request = worker.slice(worker.indexOf("function requestDirectoryAdministrationCapability"), worker.indexOf("function settleDirectoryAdministrationCapability"));
  const settle = worker.slice(worker.indexOf("function settleDirectoryAdministrationCapability"), worker.indexOf("function closeDirectoryAdministration"));
  assert(request.includes('inviteCapabilityStatus = "copying"') && request.includes('kind: "directory-administration-capability"'));
  assert(request.includes("operation.authorPage") && request.indexOf("operation.authorPage") < request.indexOf('kind: "directory-administration-capability"'));
  assert(request.includes("if (!operation.authorPage)") && request.includes("operation.inviteToken = null"));
  assert(settle.includes("if (copied)") && settle.includes("operation.inviteToken = null") && settle.includes('inviteCapabilityStatus = "failed"'));
  assert(worker.includes('kind: "directory-administration-capability-rejected"') && worker.includes('code: "mismatch"') && worker.includes('code: "already-settled"'));
  assert(shell.includes('kind: "directory-administration-capability-result"') && shell.includes(".then((copied)") && shell.includes("if (clipboard === undefined) return false"));
  assert(shell.includes("shellSpaceAdministrationOpening") && shell.includes("spaceAdministrationRef.current") && shell.includes("shellSpaceAdministrationCapabilityAllowed"));
  assert(!shell.includes("console.log(inviteToken)") && !shell.includes("setInviteToken"));
  assert(pane.includes('inviteCapabilityStatus === "copying"') && pane.includes("labels.copyStatus[inviteCapabilityStatus]"));
  return 21;
}
/** 👥️ Proves exact-scope lifecycle keys and worker-verified host-only presence projection. */
export function scopedPresenceOracle(repoRoot: string): number {
  const shellHostRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost");
  const contractRoot = join(shellHostRoot, "👥️presence-scope");
  const fixture = JSON.parse(readFileSync(join(shellHostRoot, "🧪️tests/🧫️fixtures/👥️presence-scope/🔣️.json"), "utf8")) as {
    documentId: string;
    cases: readonly { readonly scope: { readonly spaceId: string; readonly documentId: string }; readonly runtimeKey: string; readonly surface: string }[];
    close: { readonly clearsRuntimeKey: string; readonly preservesRuntimeKey: string };
    routes: readonly string[];
    rejected: readonly string[];
  };
  const bindingExport = ownedExport(repoRoot, "presence", "ScopedPresenceBindingV1");
  const routeExport = ownedExport(repoRoot, "presence", "PresenceActorRouteV1");
  const rejectionExport = ownedExport(repoRoot, "presence", "ScopedPresenceRejectionV1");
  for (const row of fixture.cases) assert(bindingExport(row), JSON.stringify(bindingExport.errors));
  assert(fixture.cases.some((row) => row.runtimeKey === fixture.close.clearsRuntimeKey) && fixture.cases.some((row) => row.runtimeKey === fixture.close.preservesRuntimeKey));
  for (const route of fixture.routes) assert(routeExport(route), JSON.stringify(routeExport.errors));
  for (const rejected of fixture.rejected) assert(rejectionExport(rejected), JSON.stringify(rejectionExport.errors));
  const runtimeKey = (scope: { readonly spaceId: string; readonly documentId: string }): string => {
    const spaceBytes = new TextEncoder().encode(scope.spaceId).length;
    const documentBytes = new TextEncoder().encode(scope.documentId).length;
    return `v1:${spaceBytes}:${documentBytes}:${scope.spaceId}${scope.documentId}`;
  };
  assert(fixture.cases.every((row) => row.scope.documentId === fixture.documentId && runtimeKey(row.scope) === row.runtimeKey));
  assert.equal(new Set(fixture.cases.map((row) => row.scope.spaceId)).size, 2);
  assert.deepEqual(
    fixture.routes,
    fixture.cases.map((row) => `actor://${row.runtimeKey}`),
  );
  assert.notEqual(fixture.close.clearsRuntimeKey, fixture.close.preservesRuntimeKey);
  assert.deepEqual(fixture.rejected, ["missing-scope", "mismatched-scope", "missing-surface", "mismatched-surface"]);
  const shell = readModuleTreeSource(join(contractRoot, "../🟦️.tsx"));
  const hostBootstrap = readFileSync(join(contractRoot, "../🪪️host-bootstrap/🟦️.tsx"), "utf8");
  const projection = readFileSync(join(contractRoot, "🟦️.ts"), "utf8");
  const browser = readFileSync(join(contractRoot, "🌐️browser/🟦️.tsx"), "utf8");
  const worker = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"));
  const wire = readModuleTreeSource(join(repoRoot, "🧰️framework/🔨️modules/🛰️backbone/🧱️worker-wire/🟦️.ts"));
  assert(shell.includes("scopedPresencePeersV1") && shell.includes("presencePeersByRuntimeKey"));
  assert(shell.includes("portByRuntimeKey") && shell.includes("operationRuntimeKey") && !shell.includes("portByDocumentId"));
  assert(shell.includes("inferencePortOwnerRef") && shell.includes("owners.length === 1"));
  assert(shell.includes("inferencePortStatusRuntimeKeyV1(inferencePortOwnerRef.current") && hostBootstrap.includes("message.scope.spaceId !== owner.scope.spaceId") && hostBootstrap.includes("message.scope.documentId !== owner.scope.documentId"));
  assert(shell.includes("retainInferencePortOwnerAfterCloseV1(inferenceOwner, runtimeKey)") && !worker.includes("inferencePort"));
  assert(shell.includes('from "../👥️presence-scope/🟦️.ts"'));
  assert(!shell.includes("presencePeersJson"));
  assert(shell.includes("entry.plugin.bindDocumentPort(entry.session.instanceId") && !shell.includes("registerPluginBackboneRoute("));
  assert(shell.includes("documentId: entry.documentId") && shell.includes("spaceId: entry.scope.spaceId"));
  assert(shell.includes('message: { kind: "documentBackbone", message }') && shell.includes("receiveDocumentBackbone(runtimeKey, entry, event.message)"));
  assert(worker.includes("presenceCandidate") && worker.includes("presenceAuthority"));
  assert(worker.includes('emitEvent(state, { kind: "presence", peers: [] })'));
  assert(worker.includes("const scope = artifactScope(state)"));
  assert(wire.includes("workerWireScopeV1") && worker.includes("event: verified ? event : { ...event, peers: [] }"));
  assert(projection.includes("message.verifiedSurfaceId !== expectedSurfaceId") && !projection.includes("peer.surface ==="));
  assert(projection.includes('role === "owner" || role === "member"') && projection.includes('role === "viewer"'));
  assert(browser.includes('new Worker(config.workerUrl, { type: "module" })'));
  assert(browser.includes("new MessageChannel()") && browser.includes("for (const row of config.cases)"));
  assert(browser.includes('kind: "close"') && browser.includes('kind: "presenceHeartbeat"'));
  assert(browser.includes('data-shell-host="scoped-presence"'));
  return 29;
}
