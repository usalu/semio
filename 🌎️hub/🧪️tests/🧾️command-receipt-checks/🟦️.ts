/** 🧭️ Directory command receipt and authority gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { type CompilerAdmissionControl } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { type ScriptInvocation } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { directoryCommandRequestJson, directoryCommandSha256, parseDirectoryCommandReceiptV1, parseDirectoryCommandRequestV1, sealDirectoryCommandRequestV1 } from "@semio-tech/framework/directory/io/text";
import { type ExactCargoLawGroup } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import { directoryCommandErrorFromStatus, directoryCommandErrorIsTransient } from "@semio-tech/framework/directory/schema";
import type { DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1 } from "@semio-tech/framework/directory/schema";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { runCmd } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { issueLocalCredential } from "../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { type LocalProfile } from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { finishLocalHub, type LocalHubRun, withPublishedLocalHubHostV1, startLocalHub, waitForReadiness } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { createdLiveDirectorySpace, liveDirectoryEventPageUser, submitLiveDirectoryCommand } from "../📣️directory-live-support/🟦️.ts";
import { proveInviteRedemptionTransaction } from "../🎫️invite-redemption-checks/🟦️.ts";
import { liveDirectoryCommandRequestId, postLiveDirectoryCommand } from "../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";
import { hubLawGroups, hubOracleFunction, hubOracleNumber, readDirectoryBackendSource, requireHubDatum } from "../../🧪️testing/📖️source-readers/🟦️.ts";

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
async function proveDirectoryCommandReceiptV1Process(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl): Promise<void> {
  return withPublishedLocalHubHostV1(invocation,phaseControl,async(host,context)=>{
  const {workspace:repoRoot,root,dataRoot}=context;
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot || !isAbsolute(artifactRoot)) throw new Error("directory command receipt process requires an absolute ticket-local SEMIO_TEST_ARTIFACT_DIR");
  const profiles: readonly LocalProfile[] = [
    { profileId: "command-receipt-author", subject: "command-receipt-process-author", displayName: "Command Receipt Author", allowedClientClasses: ["native"] },
    { profileId: "command-receipt-spectator", subject: "command-receipt-process-spectator", displayName: "Command Receipt Spectator", allowedClientClasses: ["native"] },
  ];
  let run: LocalHubRun | undefined;
  let author: Record<string, any> | undefined;
  let spectator: Record<string, any> | undefined;
  try {
    run = await startLocalHub(host, profiles, { capture: true, isolatedSecuritySmoke: true });
    await waitForReadiness(run, true);
    author = await issueLocalCredential(run, profiles[0]!.profileId, "native", 2);
    spectator = await issueLocalCredential(run, profiles[1]!.profileId, "native", 3);
    const spectatorUser = await liveDirectoryEventPageUser(run, spectator);
    const spaceId = createdLiveDirectorySpace(await submitLiveDirectoryCommand(run, author, { kind: "create-space", name: "Command Receipt Space", spaceKind: "studio", visibility: "private" }));
    await submitLiveDirectoryCommand(run, author, { kind: "upsert-member", spaceId, email: spectatorUser.email, role: "spectator" });

    const inviteCommand = { kind: "create-invite", spaceId, role: "spectator", ttlSecs: 3600 };
    const requestId = liveDirectoryCommandRequestId();
    const first = await postLiveDirectoryCommand(run, author.capability, requestId, inviteCommand);
    const firstReceipt = first.status === 202 ? (JSON.parse(first.text) as Record<string, any>) : undefined;
    const token = firstReceipt?.result?.inviteToken;
    if (!firstReceipt || firstReceipt.outcome !== "accepted" || firstReceipt.requestId !== requestId || typeof token !== "string" || token.length === 0) {
      throw new Error(`directory command receipt process first invite did not carry exactly one capability: ${first.status}`);
    }

    const retry = await postLiveDirectoryCommand(run, author.capability, requestId, inviteCommand);
    const retryReceipt = retry.status === 202 ? (JSON.parse(retry.text) as Record<string, any>) : undefined;
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

    const oversize = await postLiveDirectoryCommand(
      run,
      author.capability,
      liveDirectoryCommandRequestId(),
      inviteCommand,
      JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: liveDirectoryCommandRequestId(), command: { kind: "rename-space", spaceId, name: "x".repeat(9 * 1024) } }),
    );
    if (oversize.status !== 413 && oversize.status !== 400) throw new Error(`directory command receipt process did not bound an oversize request: ${oversize.status}`);

    await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me/sign-out`, { method: "POST", headers: { authorization: `Bearer ${author.capability}` }, signal: AbortSignal.timeout(5_000) });
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
  });
}

/** 🧾️ Language-agnostic vectors for the closed directory command request/receipt wire. */
type DirectoryCommandReceiptFixture = {
  readonly schema: "semio.hub.directory-command-receipt/v1";
  readonly limits: { readonly requestBytes: 8192; readonly receiptBytes: 65536; readonly maxEvents: 4; readonly inviteTokenBytes: 256; readonly requestIdLen: 32 };
  readonly requests: readonly { readonly name: string; readonly requestId: string; readonly command: DirectoryCommand; readonly canonical: string; readonly canonicalBytes: number; readonly commandSha256: string }[];
  readonly receipts: readonly {
    readonly name: string;
    readonly requestName: string;
    readonly outcome: DirectoryCommandOutcomeV1;
    readonly canonical: string;
    readonly canonicalBytes: number;
    readonly receiptSha256: string;
    readonly receipt: DirectoryCommandReceiptV1;
  }[];
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
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/🧾️command-receipt-v1");
  const source = readFileSync(join(root, "🔣️.json"), "utf8");
  const mirror = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧾️command-receipt-v1.json"), "utf8");
  if (source !== mirror) throw new Error("directory command receipt fixture drifted between the hub and os trees");
  const fixture = JSON.parse(source) as DirectoryCommandReceiptFixture;
  if (fixture.schema !== "semio.hub.directory-command-receipt/v1") throw new Error("directory command receipt fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "limits,receipts,rejectedReceipts,rejectedRequests,requests,schema,transport") throw new Error("directory command receipt fixture envelope drift");
  if (fixture.limits.requestBytes !== 8192 || fixture.limits.receiptBytes !== 65536 || fixture.limits.maxEvents !== 4 || fixture.limits.inviteTokenBytes !== 256 || fixture.limits.requestIdLen !== 32) throw new Error("directory command receipt bound drift");
  if (fixture.requests.length < 4 || fixture.receipts.length < 5 || fixture.rejectedRequests.length < 8 || fixture.rejectedReceipts.length < 12 || fixture.transport.traces.length < 6) throw new Error("directory command receipt table sizes drifted");
  if (fixture.transport.capacity !== 64 || fixture.transport.statusCodes.length < 8 || fixture.transport.transientCodes.length < 2 || fixture.transport.terminalCodes.length < 9) throw new Error("directory command receipt transport inventory drift");
  const receiptContract = hubSchemaExport(repoRoot, "schema://hub.directory/DirectoryCommandReceiptV1");
  for (const row of fixture.receipts) {
    if (!receiptContract(row.receipt)) throw new Error(`directory command receipt is not the owned receipt contract: ${row.name}`);
    if (receiptContract({ ...row.receipt, extra: true })) throw new Error(`directory command receipt contract admitted an unknown field: ${row.name}`);
  }
  const digest = (text: string): string => createHash("sha256").update(Buffer.from(text, "utf8")).digest("hex");
  const bytes = (text: string): number => Buffer.byteLength(text, "utf8");
  let checks = 1;

  const parsedRequests = new Map<string, DirectoryCommandRequestV1>();
  for (const request of fixture.requests) {
    const canonical = JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: request.requestId, command: request.command });
    if (canonical !== request.canonical || bytes(canonical) !== request.canonicalBytes || request.canonicalBytes > fixture.limits.requestBytes)
      throw new Error(`directory command request '${request.name}' is not canonical within the request ceiling`);
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
    if (JSON.stringify(receipt.receipt) !== receipt.canonical || bytes(receipt.canonical) !== receipt.canonicalBytes || receipt.canonicalBytes > fixture.limits.receiptBytes)
      throw new Error(`directory command receipt '${receipt.name}' is not canonical within the receipt ceiling`);
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
    if (parsed.receiptSha256 !== receipt.receiptSha256 || parsed.commandSha256 !== fixture.requests.find((entry) => entry.name === receipt.requestName)?.commandSha256)
      throw new Error(`directory command receipt '${receipt.name}' does not round-trip`);
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

/** 🛡️ Cross-checks neutral command/revocation order with an independent collection oracle. */
async function proveDirectoryCommandAuthorityV1(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/🛡️command-authority-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  if (fixture.version !== 1) throw new Error("directory command authority fixture version drift");
  if (Object.keys(fixture).sort().join(",") !== "bindings,capacity,cases,inviteScopes,version") throw new Error("directory command authority fixture envelope drift");
  if (fixture.capacity.subjects !== 65 || fixture.capacity.subjectLimit !== 64 || fixture.capacity.ledgerLimit !== 4096 || fixture.capacity.accepted !== 65) throw new Error("directory command authority capacity drift");
  if (fixture.cases.length < 4 || fixture.bindings.length < 5 || fixture.inviteScopes.length !== 4) throw new Error("directory command authority table sizes drifted");
  if (new Set(fixture.cases.map((row: { id: string }) => row.id)).size !== fixture.cases.length || new Set(fixture.bindings.map((row: { id: string }) => row.id)).size !== fixture.bindings.length) throw new Error("directory command authority ids are not unique");
  if (fixture.inviteScopes.map((row: { id: string }) => row.id).join(",") !== "foreign-invite,foreign-accepted-invite,owned-invite,owned-accepted-invite") throw new Error("directory command authority invite-scope inventory drift");
  for (const row of fixture.cases) {
    if (!["demote", "remove", "session"].includes(row.revocation) || ![202, 401, 403].includes(row.status) || ![0, 1].includes(row.appended)) throw new Error("directory command authority case taxonomy drift: " + row.id);
    if (row.events.length !== 3 || new Set(row.events).size !== 3 || row.events.some((event: string) => !["authorize", "revoke", "fence"].includes(event))) throw new Error("directory command authority case events drift: " + row.id);
  }
  for (const row of fixture.bindings) {
    if (!["document", "scoped", "global"].includes(row.audience) || !["author", "owner"].includes(row.user) || !["changed", "other"].includes(row.space) || typeof row.invalidated !== "boolean") throw new Error("directory command authority binding taxonomy drift: " + row.id);
  }
  for (const row of fixture.inviteScopes) {
    if (!["owned", "foreign"].includes(row.commandSpace) || row.inviteSpace !== "foreign" || ![404, 202, 409].includes(row.status) || typeof row.revoked !== "boolean" || typeof row.accepted !== "boolean") throw new Error("directory command authority invite-scope taxonomy drift: " + row.id);
  }
  const findIndex = await hubOracleFunction("lodash-es/findIndex.js");
  for (const row of fixture.cases) {
    let revoked = false,
      status = 0;
    for (const event of row.events) {
      if (event === "revoke") revoked = true;
      if (event === "fence") status = revoked ? (row.revocation === "session" ? 401 : 403) : 202;
    }
    const revokedFirst = hubOracleNumber(findIndex(row.events, (event: string) => event === "revoke")) < hubOracleNumber(findIndex(row.events, (event: string) => event === "fence"));
    const expected = revokedFirst ? (row.revocation === "session" ? 401 : 403) : 202;
    if (status !== expected || status !== row.status || Number(status === 202) !== row.appended) throw new Error("directory authority trace: " + row.id);
  }
  const every = await hubOracleFunction("lodash-es/every.js");
  for (const row of fixture.bindings) {
    const invalidated = row.user === "author" && row.space === "changed" && row.audience !== "global";
    const expected = every([row.user === "author", row.space === "changed", row.audience !== "global"], Boolean);
    if (invalidated !== expected || invalidated !== row.invalidated) throw new Error("directory binding trace: " + row.id);
  }
  const expectedCases = ["command-before-demotion", "demotion-before-command", "removal-before-command", "session-before-command"];
  const expectedBindings = ["directory-author", "document-author", "global-author", "other-space", "other-user"];
  if (JSON.stringify(fixture.cases.map((row: { id: string }) => row.id).sort()) !== JSON.stringify(expectedCases) || JSON.stringify(fixture.bindings.map((row: { id: string }) => row.id).sort()) !== JSON.stringify(expectedBindings))
    throw new Error("directory authority corpus is incomplete");
  const sum = await hubOracleFunction("lodash-es/sum.js");
  const admissions = Array.from({ length: fixture.capacity.subjects }, () => 1);
  if (!admissions.every((count) => count <= fixture.capacity.subjectLimit) || hubOracleNumber(sum(admissions)) > fixture.capacity.ledgerLimit || sum(admissions) !== fixture.capacity.accepted) throw new Error("directory authority shared-space capacity");
  const expectedInvites = ["foreign-accepted-invite", "foreign-invite", "owned-accepted-invite", "owned-invite"];
  if (JSON.stringify(fixture.inviteScopes.map((row: { id: string }) => row.id).sort()) !== JSON.stringify(expectedInvites)) throw new Error("directory invite corpus is incomplete");
  for (const row of fixture.inviteScopes) {
    const exact = row.commandSpace === row.inviteSpace;
    const expected = every([row.commandSpace === row.inviteSpace, !row.accepted], Boolean);
    if (row.revoked !== expected || row.status !== (exact ? (row.accepted ? 409 : 202) : 404)) throw new Error("directory invite scope trace: " + row.id);
  }

  const backend = readDirectoryBackendSource(repoRoot, "🪶️sqlite");
  const revoke = backend.slice(backend.indexOf("    async fn revoke_invite_as("), backend.indexOf("    async fn list_invites("));
  const update = /tx\.execute\("([^"]+)",\s*rusqlite::params!\[([^\]]+)\]/.exec(revoke);
  if (!update || !requireHubDatum(update[1],"update[1]").startsWith("UPDATE hub_space_invite ")) throw new Error("directory invite revoke query is unavailable");
  const lookup = /tx\.query_row\("([^"]+)",\s*rusqlite::params!\[([^\]]+)\]/.exec(revoke);
  if (!lookup || !requireHubDatum(lookup[1],"lookup[1]").startsWith("SELECT accepted_at ")) throw new Error("directory invite accepted lookup is unavailable");
  const { Database } = await import("bun:sqlite");
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE hub_space_invite (id TEXT PRIMARY KEY, space_id TEXT NOT NULL, revoked_at INTEGER, revoked_reason TEXT, accepted_at INTEGER)");
    for (const row of fixture.inviteScopes) {
      database.query("DELETE FROM hub_space_invite").run();
      database.query("INSERT INTO hub_space_invite (id, space_id, accepted_at) VALUES (?, ?, ?)").run("invite-foreign", "foreign", row.accepted ? 7 : null);
      const values: Record<string, string | number> = { invite_id: "invite-foreign", space_id: row.commandSpace, revoked_at: 42, reason: "contract-test" };
      const params = (names: string) =>
        names.split(",").map((name) => {
          const value = values[name.trim()];
          if (value === undefined) throw new Error("directory invite query has an unqualified parameter");
          return value;
        });
      const result = database.query(requireHubDatum(update[1],"update[1]")).run(...params(requireHubDatum(update[2],"update[2]")));
      const stored = database.query("SELECT revoked_at FROM hub_space_invite WHERE id = ?").get("invite-foreign") as { revoked_at: number | null };
      if (result.changes !== Number(row.revoked) || (stored.revoked_at !== null) !== row.revoked) throw new Error("production SQLite invite scope: " + row.id);
      const accepted = database.query(requireHubDatum(lookup[1],"lookup[1]")).get(...params(requireHubDatum(lookup[2],"lookup[2]"))) as { accepted_at: number | null } | null;
      const status = result.changes === 1 ? 202 : accepted?.accepted_at != null ? 409 : 404;
      if (status !== row.status) throw new Error("production SQLite accepted-invite scope: " + row.id);
      console.log(`directory-invite-sqlite case=${row.id} changed=${result.changes} revoked=${stored.revoked_at !== null}`);
    }
  } finally {
    database.close();
  }

  for (const kind of ["🐘️postgres", "🌐️neo4j"]) {
    const source = readDirectoryBackendSource(repoRoot, kind);
    const method = source.slice(source.indexOf("    async fn revoke_invite_as("), source.indexOf("    async fn list_invites("));
    if (kind === "🐘️postgres") {
      const parameters = Array.from(method.matchAll(/\.bind\((\w+)\)/g), (match) => match[1]);
      if (
        !method.includes("WHERE space_id = $4 AND id = $1 AND revoked_at IS NULL AND accepted_at IS NULL") ||
        !method.includes("WHERE space_id = $2 AND id = $1") ||
        JSON.stringify(parameters) !== JSON.stringify(["invite_id", "revoked_at", "reason", "space_id", "invite_id", "space_id"])
      )
        throw new Error("PostgreSQL invite scope predicates or bindings drifted");
    } else {
      if (method.split("MATCH (i:SpaceInvite {spaceId: $space_id, id: $id})").length !== 3 || method.split('.param("space_id", space_id)').length !== 3) throw new Error("Neo4j invite scope predicates or bindings drifted");
    }
  }
  return fixture.cases.length + fixture.bindings.length + fixture.inviteScopes.length * 3 + 3;
}

export class DirectoryCommandReceiptCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "authority", "invites", "native", "process"].includes(phase)) throw new Error("directory-command-receipt-check accepts source, authority, invites, native, or process");
    const checks = (await proveDirectoryCommandReceiptV1(this.repoRoot)) + (await proveDirectoryCommandAuthorityV1(this.repoRoot)) + (await proveInviteRedemptionTransaction(this.repoRoot));
    if (phase === "authority" || phase === "invites" || phase === "native" || phase === "process") {
      const laws: [string, string][] = [["semio-hub-directory-route-laws", "directory_command_authority_revalidates_after_durable_revocation_before_fence"], ["semio-hub-directory-route-laws", "directory_command_authority_holds_admitted_command_until_receipt_before_demotion"], ["semio-hub-directory-route-laws", "directory_command_authority_demotion_invalidates_only_affected_scope_once"], ["semio-hub-directory-route-laws", "directory_command_authority_invite_revocation_requires_the_exact_owned_space"], ["semio-hub-directory-route-laws", "directory_invite_redemption_obeys_current_space_state_and_readonly_replay"], ["semio-hub-directory-route-laws", "directory_invite_redemption_revalidates_after_hint_before_fence"], ["semio-hub-directory-route-laws", "directory_invite_redemption_admitted_fence_precedes_archive"], ["semio-hub-directory-route-laws", "directory_invite_redemption_scope_hint_is_capability_bound"], ["semio-hub-directory-route-laws", "directory_command_receipt_v1_route_is_request_idempotent_for_concurrent_identical_ids"], ["semio-hub-directory-route-laws", "directory_command_receipt_v1_route_denies_cross_user_spectator_and_digest_substitution"], ["semio-hub-directory-route-laws", "directory_command_receipt_v1_route_bounds_request_and_receipt_bytes"], ["semio-hub-directory-route-laws", "directory_command_receipt_v1_store_resolves_a_lost_reply_and_survives_restart"]];
      for (const group of ([
        ...hubLawGroups(phase === "invites" ? laws.slice(4, 8) : laws.slice(0, 8)),
        ...(phase === "invites"
          ? [
              {
                package: "semio-hub-directory-laws",
                target: { kind: "lib" },
                laws: ["unit::invite_redemption_sqlite_claim_is_exactly_once_across_concurrency_restart_and_rebuild", "unit::invite_redemption_projection_failure_rolls_back_claim_event_and_membership", "unit::invite_redemption_commit_and_publication_precede_the_next_directory_command", "unit::invite_archive_projection_serializes_independent_service_decisions", "unit::directory_command_uncertain_commit_retains_claim_and_never_reexecutes"],
              },
            ] satisfies ExactCargoLawGroup[]
          : []),
        ...(["authority", "invites"].includes(phase) ? [] : hubLawGroups(laws.slice(8), ["--all-features"])),
      ] satisfies ExactCargoLawGroup[]).sort((left, right) => Number(left.target.kind === "bin") - Number(right.target.kind === "bin"))) {
        const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.root,...exactCargoStageEnvironments(),groups: [group],progress(event) {
            console.log(`directory-command-receipt-${phase} ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`);
          }});
        for (const receipt of receipts) console.log(`directory-command-receipt-${phase}-receipt: ${JSON.stringify(receipt)}`);
      }
      if (phase === "process") {
        runCmd("cargo", ["build", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
        await proveDirectoryCommandReceiptV1Process(this.invocation,exactCargoCallerControlV1(this.invocation,"hub:command-receipt",0));
      }
    }
    console.log(`directory-command-receipt-check: checks=${checks} phase=${phase}`);
  }
}
