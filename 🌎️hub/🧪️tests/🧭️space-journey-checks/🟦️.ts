/** 🧭️ Directory space journey fixture and process gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { type CompilerAdmissionControl } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { type ScriptInvocation } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { directoryCommandRequestJson, parseDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1 } from "@semio-tech/framework/directory/io/text";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import type { DirectoryCommand } from "@semio-tech/framework/directory/schema";
import { runCmd } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { readHubDirectorySource } from "../../📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts";
import { issueLocalCredential } from "../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { type LocalProfile } from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { finishLocalHub, hubBinaryPath, type LocalHubRun, withPublishedLocalHubHostV1, startLocalHub, waitForReadiness } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { readHubScriptModule } from "../../🧪️testing/📜️script-sources/🟦️.ts";
import { createdLiveDirectorySpace, fetchLiveDirectoryEventPage, liveDirectoryEventPageUser } from "../📣️directory-live-support/🟦️.ts";
import { liveDirectoryCommandRequestId, postLiveDirectoryCommand } from "../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";
import { readHubBinSource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

//#region 🔖️SpaceJourney
/** 🚻️ Language-neutral two-user space journey: the ordered step table, its honest skips, and the
 * EN/DE label bundle every emitted trace line carries with no default language. */
type DirectorySpaceJourneyFixture = {
  readonly schema: "semio.hub.directory-space-journey/v1";
  readonly limits: { readonly journeyMs: number; readonly stepMs: number; readonly responseBytes: 65536; readonly commandRequestBytes: 8192; readonly administrationPageBytes: 49152; readonly socketWaitMs: number };
  readonly bindings: Readonly<
    Record<"eventPageDomain" | "administrationDomain" | "domainTerminator" | "commandRequestSchema" | "commandReceiptSchema" | "eventPageSchema" | "administrationPageSchema" | "socketProtocol" | "socketHelloSchema", string>
  >;
  readonly profiles: Readonly<Record<"a" | "b", { readonly profileId: string; readonly subject: string; readonly displayName: string }>>;
  readonly space: { readonly name: string; readonly spaceKind: "atelier" | "studio" | "archive"; readonly visibility: "private" | "public"; readonly renamedName: string; readonly privateBName: string };
  readonly document: {
    readonly documentId: string;
    readonly artifactKind: string;
    readonly artifactSchema: string;
    readonly owner: { readonly pluginId: string; readonly packageId: string; readonly version: string; readonly packageHash: string };
    readonly packSchemaHash: string;
    readonly bootstrapVersion: number;
    readonly bootstrapFrontier: { readonly headSeq: number; readonly commitSeq: number; readonly epoch: number };
    readonly bootstrapSnapshotHash: string;
  };
  readonly steps: readonly {
    readonly id: string;
    readonly actor: "a" | "b" | "both";
    readonly route: string;
    readonly method: "GET" | "POST";
    readonly expect: Record<string, any>;
    readonly labels: { readonly en: string; readonly de: string };
  }[];
  readonly skips: readonly { readonly id: string; readonly reason: string; readonly admissibleStatuses: readonly number[]; readonly labels: { readonly en: string; readonly de: string } }[];
  readonly privacy: { readonly forbiddenTraceSubstrings: readonly string[]; readonly forbiddenPageSubstrings: readonly string[] };
  readonly routes: readonly string[];
};

const SPACE_JOURNEY_ORDER = [
  "create-space",
  "create-space-redacted-retry",
  "author-reads-administration-page",
  "nonmember-administration-page-denied",
  "author-adds-b-as-spectator",
  "member-reads-administration-page",
  "author-promotes-b",
  "announce-document",
  "b-creates-private-space",
  "open-global-sockets",
  "open-scoped-socket",
  "member-renames-space",
  "author-observes-dirty-wake",
  "author-reads-ordered-event-page",
  "author-removes-b",
  "scoped-socket-revoked",
  "removed-member-page-denied",
  "author-self-revokes",
  "self-revoked-read-denied",
  "restart-preserves-history",
] as const;

function spaceJourneyStep(fixture: DirectorySpaceJourneyFixture, id: string): DirectorySpaceJourneyFixture["steps"][number] {
  const step = fixture.steps.find((entry) => entry.id === id);
  if (!step) throw new Error(`space journey fixture declares no step '${id}'`);
  return step;
}

function spaceJourneySkip(fixture: DirectorySpaceJourneyFixture, id: string): DirectorySpaceJourneyFixture["skips"][number] {
  const skip = fixture.skips.find((entry) => entry.id === id);
  if (!skip) throw new Error(`space journey fixture declares no skip '${id}'`);
  return skip;
}

/** 🔐️ Independently rebuilds the administration page's domain-separated session binding from the
 * fixture's declared domain, so the hub's digest is never its own witness. */
function spaceJourneyAdministrationBinding(domain: string, sessionId: string, userId: string, generation: number, expiresAt: number, spaceId: string): string {
  const u32 = (value: number): Buffer => {
    const bytes = Buffer.alloc(4);
    bytes.writeUInt32BE(value);
    return bytes;
  };
  const u64 = (value: number): Buffer => {
    const bytes = Buffer.alloc(8);
    bytes.writeBigUInt64BE(BigInt(value));
    return bytes;
  };
  const i64 = (value: number): Buffer => {
    const bytes = Buffer.alloc(8);
    bytes.writeBigInt64BE(BigInt(value));
    return bytes;
  };
  const session = Buffer.from(sessionId, "utf8");
  const user = Buffer.from(userId, "utf8");
  const space = Buffer.from(spaceId, "utf8");
  return createHash("sha256")
    .update(Buffer.concat([Buffer.from(`${domain}\0`, "utf8"), u32(session.length), session, u32(user.length), user, u64(generation), i64(expiresAt), u32(space.length), space]))
    .digest("hex");
}

/** 📡️ Rebuilds the exact tag-7 directory socket hello frame the shared client sends. */
function spaceJourneySocketHelloFrame(schema: string): Buffer {
  const bytes = Buffer.from(schema, "utf8");
  if (bytes.length === 0 || bytes.length > 255) throw new Error("space journey socket hello schema is not one length-prefixed byte");
  return Buffer.concat([Buffer.from([0, 7, 1, 1, bytes.length]), bytes, Buffer.alloc(32), Buffer.from([0, 0])]);
}

/** ⚖️ The fixture-shape laws, applied to the published vectors and to every hostile mutation so a
 * removed law is provably armed rather than merely present. */
function spaceJourneyFixtureLaws(fixture: DirectorySpaceJourneyFixture, hub: string, runnerBody: string): number {
  let checks = 0;
  const stepIds = fixture.steps.map((step) => step.id);
  if (JSON.stringify(stepIds) !== JSON.stringify([...SPACE_JOURNEY_ORDER])) throw new Error(`space journey step order differs from the runner's declared order: ${JSON.stringify(stepIds)}`);
  if (new Set(stepIds).size !== stepIds.length) throw new Error("space journey step ids repeat");
  const skipIds = fixture.skips.map((skip) => skip.id);
  if (new Set(skipIds).size !== skipIds.length || skipIds.some((id) => stepIds.includes(id))) throw new Error("space journey skip ids repeat or collide with a step id");
  checks += 3;
  for (const entry of [...fixture.steps, ...fixture.skips]) {
    if (entry.labels.en === entry.labels.de || entry.labels.en.length === 0 || entry.labels.de.length === 0) throw new Error(`space journey entry '${entry.id}' has no distinct English and German label`);
    if (entry.labels.en.trim() !== entry.labels.en || entry.labels.de.trim() !== entry.labels.de) throw new Error(`space journey entry '${entry.id}' label is not trimmed`);
    checks += 2;
  }
  for (const id of [...stepIds, ...skipIds]) {
    if (!runnerBody.includes(`"${id}"`)) throw new Error(`space journey runner never references entry '${id}'`);
    checks += 1;
  }
  for (const route of fixture.routes) {
    if (!hub.includes(`.route("${route}"`)) throw new Error(`space journey names route '${route}' which the hub does not register`);
    checks += 1;
  }
  for (const step of fixture.steps) {
    if (!fixture.routes.includes(step.route)) throw new Error(`space journey step '${step.id}' names an unregistered route`);
    checks += 1;
  }
  for (const skip of fixture.skips) {
    if (skip.reason.length < 24 || skip.admissibleStatuses.some((status) => status < 400)) throw new Error(`space journey skip '${skip.id}' is not an honest denial`);
    checks += 1;
  }
  return checks;
}

/** 🗡️ One mutation per fixture-shape law; each must be rejected by `spaceJourneyFixtureLaws`. */
function spaceJourneyHostiles(fixture: DirectorySpaceJourneyFixture): readonly { readonly name: string; readonly fixture: DirectorySpaceJourneyFixture }[] {
  const clone = (): any => JSON.parse(JSON.stringify(fixture));
  const reordered = clone();
  [reordered.steps[0], reordered.steps[1]] = [reordered.steps[1], reordered.steps[0]];
  const repeated = clone();
  repeated.skips[1].id = repeated.skips[0].id;
  const collided = clone();
  collided.skips[0].id = collided.steps[0].id;
  const defaulted = clone();
  defaulted.steps[0].labels.de = defaulted.steps[0].labels.en;
  const untrimmed = clone();
  untrimmed.steps[0].labels.de = ` ${untrimmed.steps[0].labels.de}`;
  const unreferenced = clone();
  unreferenced.skips[0].id = "never-referenced-skip";
  const unregistered = clone();
  unregistered.routes.push("/directory/forged");
  const foreign = clone();
  foreign.steps[0].route = "/spaces/{space_id}/documents/{id}/open-plan";
  foreign.routes = foreign.routes.filter((route: string) => route !== "/spaces/{space_id}/documents/{id}/open-plan");
  const dishonest = clone();
  dishonest.skips[0].admissibleStatuses = [200];
  const terse = clone();
  terse.skips[0].reason = "because";
  return [
    { name: "reordered-steps", fixture: reordered },
    { name: "repeated-skip-id", fixture: repeated },
    { name: "skip-collides-with-step", fixture: collided },
    { name: "default-language", fixture: defaulted },
    { name: "untrimmed-label", fixture: untrimmed },
    { name: "unreferenced-skip", fixture: unreferenced },
    { name: "unregistered-route", fixture: unregistered },
    { name: "foreign-step-route", fixture: foreign },
    { name: "skip-admits-success", fixture: dishonest },
    { name: "unexplained-skip", fixture: terse },
  ];
}

/** 🧾️ Replays the two-user journey contract without the Rust implementation: AJV over the published
 * schema, an independent `node:crypto` rebuild of both session-binding domains and the socket hello
 * frame, cross-language constant equality against the shared schema twin, an independent second
 * implementation of the descriptor admission law, and source fences proving every named route and
 * step is actually owned by the hub and driven by the process runner. */
async function proveDirectorySpaceJourneyV1(repoRoot: string): Promise<Readonly<{ fixture: DirectorySpaceJourneyFixture; checks: number }>> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/🚻️space-journey-v1");
  const source = readFileSync(join(root, "🔣️.json"), "utf8");
  const fixture = JSON.parse(source) as DirectorySpaceJourneyFixture;
  if (fixture.schema !== "semio.hub.directory-space-journey/v1") throw new Error("space journey fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "bindings,document,limits,privacy,profiles,routes,schema,skips,space,steps") throw new Error("space journey fixture envelope drift");
  if (fixture.limits.responseBytes !== 65536 || fixture.limits.commandRequestBytes !== 8192 || fixture.limits.administrationPageBytes !== 49152) throw new Error("space journey bound drift");
  if (fixture.limits.journeyMs < 30000 || fixture.limits.journeyMs > 600000 || fixture.limits.stepMs < 1000 || fixture.limits.stepMs > 60000 || fixture.limits.socketWaitMs < 1000 || fixture.limits.socketWaitMs > 30000) throw new Error("space journey deadline drift");
  const journeyProfile = hubSchemaExport(repoRoot, "schema://hub.directory/DirectoryProfileV1");
  if (!journeyProfile(fixture.profiles.a) || !journeyProfile(fixture.profiles.b) || fixture.profiles.a.profileId === fixture.profiles.b.profileId) throw new Error("space journey profiles are not two distinct owned directory profiles");
  if (!hubSchemaExport(repoRoot, "schema://hub.directory/DirectoryDocumentDescriptorV1")(fixture.document)) throw new Error("space journey document is not the owned directory descriptor contract");
  const journeyLabels = hubSchemaExport(repoRoot, "schema://hub.directory/LocalizedTextV1");
  for (const row of [...fixture.steps, ...fixture.skips]) {
    if (!journeyLabels(row.labels)) throw new Error(`space journey labels are not the owned bilingual text contract: ${row.id}`);
    if (row.labels.en.length < 8 || row.labels.de.length < 8 || row.labels.en === row.labels.de) throw new Error(`space journey labels are not two distinct meaningful languages: ${row.id}`);
    if (!/^[a-z0-9]+(-[a-z0-9]+)*$/u.test(row.id)) throw new Error(`space journey identifier drift: ${row.id}`);
  }
  if (fixture.steps.length < 12 || fixture.steps.length > 32 || fixture.skips.length < 3 || fixture.skips.length > 16 || fixture.routes.length < 8) throw new Error("space journey inventory drift");
  if (new Set([...fixture.steps, ...fixture.skips].map((row) => row.id)).size !== fixture.steps.length + fixture.skips.length) throw new Error("space journey step identifiers are not unique");
  if (new Set(fixture.routes).size !== fixture.routes.length || fixture.routes.some((route) => !route.startsWith("/"))) throw new Error("space journey route inventory drift");
  if (fixture.steps.some((step) => !step.route.startsWith("/") || !["GET", "POST"].includes(step.method) || !["a", "b", "both"].includes(step.actor))) throw new Error("space journey step taxonomy drift");
  if (fixture.privacy.forbiddenTraceSubstrings.length < 4 || fixture.privacy.forbiddenPageSubstrings.length < 4) throw new Error("space journey privacy inventory drift");
  if (!["atelier", "studio", "archive"].includes(fixture.space.spaceKind) || !["private", "public"].includes(fixture.space.visibility)) throw new Error("space journey space taxonomy drift");
  let checks = 1;

  const runner = readHubScriptModule(repoRoot, "🧪️tests/🧭️space-journey-checks");
  const marker = runner.indexOf("\nasync function proveDirectorySpaceJourneyV1Process(");
  if (marker < 0) throw new Error("space journey process runner is not present");
  const runnerBody = runner.slice(marker, runner.indexOf("\nclass SpaceJourneyCheckScript", marker));
  const hub = readHubBinSource(repoRoot);
  checks += spaceJourneyFixtureLaws(fixture, hub, runnerBody);
  for (const hostile of spaceJourneyHostiles(fixture)) {
    let admitted = true;
    try {
      spaceJourneyFixtureLaws(hostile.fixture, hub, runnerBody);
    } catch {
      admitted = false;
    }
    if (admitted) throw new Error(`space journey hostile '${hostile.name}' was admitted`);
    checks += 1;
  }

  const schemaTwin = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"), "utf8");
  const constants: readonly (readonly [string, number])[] = [
    ["DIRECTORY_COMMAND_REQUEST_MAX_BYTES: usize = 8 * 1024", fixture.limits.commandRequestBytes],
    ["DIRECTORY_COMMAND_RECEIPT_MAX_BYTES: usize = 64 * 1024", fixture.limits.responseBytes],
    ["DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES: usize = 48 * 1024", fixture.limits.administrationPageBytes],
  ];
  for (const [declaration, expected] of constants) {
    const literal = declaration
      .split(" = ")[1]!
      .split(" * ")
      .reduce((product, part) => product * Number(part), 1);
    if (!schemaTwin.includes(declaration) || literal !== expected) throw new Error(`space journey limit ${expected} is not the shared Rust constant '${declaration}'`);
    checks += 1;
  }
  for (const [name, value] of [
    ["DIRECTORY_COMMAND_REQUEST_SCHEMA", fixture.bindings.commandRequestSchema],
    ["DIRECTORY_COMMAND_RECEIPT_SCHEMA", fixture.bindings.commandReceiptSchema],
    ["DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA", fixture.bindings.administrationPageSchema],
  ] as const) {
    if (!schemaTwin.includes(`"${value}"`)) throw new Error(`space journey binding ${name} ('${value}') is absent from the shared schema twin`);
    checks += 1;
  }

  if (fixture.bindings.domainTerminator !== "nul") throw new Error("space journey session-binding domains are not NUL-terminated");
  if (!hub.includes(`"${fixture.bindings.eventPageDomain}\\0"`) || !hub.includes(`"${fixture.bindings.administrationDomain}\\0"`)) {
    throw new Error("space journey session-binding domains are not the hub's own domain separators");
  }
  const bindingProbe = spaceJourneyAdministrationBinding(fixture.bindings.administrationDomain, "session-journey-0", "1user-journey-01", 5, 4102444800000, "space-journey-01");
  const aliasProbe = spaceJourneyAdministrationBinding(fixture.bindings.administrationDomain, "session-journey-01", "user-journey-01", 5, 4102444800000, "space-journey-01");
  if (bindingProbe === aliasProbe || !/^[0-9a-f]{64}$/.test(bindingProbe)) throw new Error("space journey administration binding is not length-prefix aliasing-safe");
  checks += 3;

  const client = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  const hello = spaceJourneySocketHelloFrame(fixture.bindings.socketHelloSchema);
  if (!client.includes(`let schema = b"${fixture.bindings.socketHelloSchema}";`) || !client.includes("bytes.extend_from_slice(&[0, 7, 1, 1, schema.len() as u8]);")) {
    throw new Error("space journey socket hello frame is not the shared client's own tag-7 encoding");
  }
  if (hello.length !== 5 + fixture.bindings.socketHelloSchema.length + 34 || hello[1] !== 7 || hello.subarray(5, 5 + fixture.bindings.socketHelloSchema.length).toString("utf8") !== fixture.bindings.socketHelloSchema) {
    throw new Error("space journey socket hello frame is not byte-exact");
  }
  if (!hub.includes(`SOCKET_PROTOCOL_V1`) || !hub.includes(`"${fixture.bindings.socketProtocol}"`)) throw new Error("space journey socket protocol is not the hub's own subprotocol");
  checks += 3;

  const descriptor = fixture.document;
  const sha256Field = (value: string): boolean => value.length === 64 && value !== "0".repeat(64) && /^[0-9a-f]{64}$/.test(value);
  if (!sha256Field(descriptor.packSchemaHash) || !sha256Field(descriptor.bootstrapSnapshotHash) || !sha256Field(descriptor.owner.packageHash)) throw new Error("space journey document descriptor carries a hash the hub would reject");
  if (descriptor.bootstrapVersion === 0 || descriptor.bootstrapFrontier.commitSeq > descriptor.bootstrapFrontier.headSeq) throw new Error("space journey document descriptor frontier is inadmissible");
  const hubDirectory = readHubDirectorySource(repoRoot);
  if (!hubDirectory.includes("pub fn validate_document_descriptor") || !hubDirectory.includes("descriptor.bootstrap_frontier.commit_seq > descriptor.bootstrap_frontier.head_seq")) {
    throw new Error("space journey descriptor admission law is no longer the hub's own");
  }
  checks += 3;

  const declared = JSON.stringify({ space: fixture.space, document: fixture.document, steps: fixture.steps, skips: fixture.skips, profiles: fixture.profiles });
  for (const forbidden of fixture.privacy.forbiddenPageSubstrings) {
    if (declared.includes(`"${forbidden}"`)) throw new Error(`space journey fixture itself declares the forbidden page field '${forbidden}'`);
    checks += 1;
  }
  const presenceSkip = spaceJourneySkip(fixture, "document-open-plan-and-presence");
  if (presenceSkip.admissibleStatuses.length === 0 || presenceSkip.admissibleStatuses.some((status) => status < 400)) throw new Error("space journey presence skip admits a success status");
  for (const id of ["document-content-edit", "rendered-surface", "invite-token-leg", "alternate-directory-backends"]) {
    if (spaceJourneySkip(fixture, id).admissibleStatuses.length !== 0) throw new Error(`space journey skip '${id}' must probe no route at all`);
    checks += 1;
  }
  checks += 1;
  return { fixture, checks };
}

type LiveDirectorySocketV1 = {
  readonly url: string;
  readonly messages: Record<string, any>[];
  readonly raw: string[];
  closeCode: number | undefined;
  readonly socket: WebSocket;
};

/** 📡️ Issues one real socket grant, dials the real hub route with it as the second subprotocol, and
 * sends the exact tag-7 hello. The grant is wiped from this driver as soon as the dial is made. */
async function openLiveDirectorySocket(run: LocalHubRun, capability: string, grantPath: string, socketUrl: string, fixture: DirectorySpaceJourneyFixture): Promise<LiveDirectorySocketV1> {
  const global = grantPath === "/directory/socket-grants";
  const response = await fetch(`http://127.0.0.1:${run.port}${grantPath}`, {
    method: "POST",
    headers: global ? { authorization: `Bearer ${capability}`, "content-type": "application/json" } : { authorization: `Bearer ${capability}` },
    body: global ? "{}" : undefined,
    signal: AbortSignal.timeout(5_000),
  });
  if (response.status !== 200) throw new Error(`space journey socket grant '${grantPath}' failed: ${response.status}`);
  const receipt = (await response.json()) as Record<string, any>;
  if (receipt.protocol !== fixture.bindings.socketProtocol || typeof receipt.grant !== "string" || receipt.grant.length === 0) throw new Error("space journey socket grant receipt was not the v1 protocol");
  const socket = new WebSocket(`ws://127.0.0.1:${run.port}${socketUrl}`, [receipt.protocol, receipt.grant]);
  receipt.grant = "";
  socket.binaryType = "arraybuffer";
  const live: LiveDirectorySocketV1 = { url: socketUrl, messages: [], raw: [], closeCode: undefined, socket };
  socket.addEventListener("message", (event: any) => {
    if (typeof event.data !== "string") return;
    live.raw.push(event.data);
    try {
      live.messages.push(JSON.parse(event.data) as Record<string, any>);
    } catch {
      live.messages.push({ kind: "undecodable" });
    }
  });
  socket.addEventListener("close", (event: any) => {
    live.closeCode = event.code;
  });
  await new Promise<void>((resolveOpen, rejectOpen) => {
    const timer = setTimeout(() => rejectOpen(new Error(`space journey socket '${socketUrl}' did not open`)), fixture.limits.socketWaitMs);
    socket.addEventListener(
      "open",
      () => {
        clearTimeout(timer);
        resolveOpen();
      },
      { once: true },
    );
    socket.addEventListener(
      "error",
      () => {
        clearTimeout(timer);
        rejectOpen(new Error(`space journey socket '${socketUrl}' failed to dial`));
      },
      { once: true },
    );
  });
  socket.send(spaceJourneySocketHelloFrame(fixture.bindings.socketHelloSchema));
  return live;
}

async function waitForLiveDirectorySocket(live: LiveDirectorySocketV1, predicate: () => boolean, budgetMs: number): Promise<boolean> {
  const deadline = Date.now() + budgetMs;
  while (Date.now() < deadline) {
    if (predicate()) return true;
    await Bun.sleep(25);
  }
  return predicate();
}

function closeLiveDirectorySocket(live: LiveDirectorySocketV1 | undefined): void {
  if (!live) return;
  try {
    if (live.socket.readyState === WebSocket.OPEN || live.socket.readyState === WebSocket.CONNECTING) live.socket.close();
  } catch {}
}

/** 📖️ Reads one caller's whole visible ordered history by following `hasMore`, returning the exact
 * events plus the raw frontier the socket must dial from. */
async function readLiveDirectoryHistory(run: LocalHubRun, envelope: Record<string, any>, user: Record<string, any>, from: number): Promise<Readonly<{ events: readonly Record<string, any>[]; frontier: number; source: string }>> {
  const events: Record<string, any>[] = [];
  const sources: string[] = [];
  let frontier = from;
  for (let page = 0; page < 64; page += 1) {
    const current = await fetchLiveDirectoryEventPage(run, envelope, user, frontier);
    sources.push(JSON.stringify(current));
    events.push(...current.events);
    frontier = current.throughSeqInclusive;
    if (!current.hasMore) break;
  }
  return { events, frontier, source: sources.join("\n") };
}

/** 🧾️ Verifies one live receipt three ways: an independent `node:crypto` recomputation of both the
 * command and receipt digests, the repository's own parser as a third implementation, and the
 * fixture's declared outcome/event/result expectation. */
async function verifyLiveSpaceJourneyReceipt(text: string, status: number, requestId: string, command: Record<string, unknown>, expect: Record<string, any>): Promise<Record<string, any>> {
  if (status !== 202) throw new Error(`space journey command was not accepted: ${status}`);
  const receipt = JSON.parse(text) as Record<string, any>;
  if (JSON.stringify(receipt) !== text) throw new Error("space journey receipt was not exact canonical JSON");
  const keys = Object.keys(receipt);
  if (JSON.stringify(keys) !== JSON.stringify(["schema", "requestId", "commandSha256", "outcome", "events", "result", "receiptSha256"])) throw new Error("space journey receipt key order is not canonical");
  const unsigned: Record<string, any> = {};
  for (const key of keys) if (key !== "receiptSha256") unsigned[key] = receipt[key];
  const digest = createHash("sha256")
    .update(Buffer.from(JSON.stringify(unsigned), "utf8"))
    .digest("hex");
  const commandDigest = createHash("sha256")
    .update(Buffer.from(JSON.stringify(command), "utf8"))
    .digest("hex");
  if (receipt.receiptSha256 !== digest || receipt.commandSha256 !== commandDigest || receipt.requestId !== requestId) throw new Error("space journey receipt digests did not verify independently");
  if (receipt.outcome !== expect.outcome || receipt.events.length !== expect.events || receipt.result?.kind !== expect.result) {
    throw new Error(`space journey receipt outcome differed: ${receipt.outcome}/${receipt.events.length}/${receipt.result?.kind}`);
  }
  if (expect.outcome !== "accepted" && (receipt.events.length !== 0 || receipt.result?.kind !== "none")) throw new Error("space journey redacted receipt carried a result");
  await parseDirectoryCommandReceiptV1(text, sealDirectoryCommandRequestV1(requestId, command as unknown as DirectoryCommand));
  let previous = 0;
  for (const event of receipt.events as Record<string, any>[]) {
    if (typeof event.seq !== "number" || event.seq <= previous) throw new Error("space journey receipt repeats or reorders a durable sequence");
    previous = event.seq;
  }
  return receipt;
}

/** 🏛️ Reads one bounded administration page and verifies its receipt and session binding without
 * trusting the hub's own digest. */
async function fetchLiveSpaceAdministrationPage(
  run: LocalHubRun,
  envelope: Record<string, any>,
  user: Record<string, any>,
  spaceId: string,
  fixture: DirectorySpaceJourneyFixture,
): Promise<Readonly<{ status: number; source: string; page?: Record<string, any> }>> {
  const response = await fetch(`http://127.0.0.1:${run.port}/directory/spaces/${encodeURIComponent(spaceId)}`, {
    headers: { authorization: `Bearer ${envelope.capability}` },
    signal: AbortSignal.timeout(5_000),
  });
  const source = await response.text();
  if (!response.ok) return { status: response.status, source };
  if (Buffer.byteLength(source, "utf8") > fixture.limits.administrationPageBytes) throw new Error("space journey administration page exceeded its bounded window");
  const page = JSON.parse(source) as Record<string, any>;
  if (JSON.stringify(page) !== source) throw new Error("space journey administration page was not exact canonical JSON");
  const keys = Object.keys(page);
  if (keys[0] !== "access" || keys[keys.length - 1] !== "receiptSha256" || page.schema !== fixture.bindings.administrationPageSchema) throw new Error("space journey administration page shape is not canonical");
  const unsigned: Record<string, any> = {};
  for (const key of keys) if (key !== "receiptSha256") unsigned[key] = page[key];
  const digest = createHash("sha256")
    .update(Buffer.from(JSON.stringify(unsigned), "utf8"))
    .digest("hex");
  const binding = spaceJourneyAdministrationBinding(fixture.bindings.administrationDomain, String(envelope.sessionId), String(user.userId), Number(user.authorizationGeneration), Number(user.expiresAt), spaceId);
  if (page.receiptSha256 !== digest || page.sessionBindingSha256 !== binding || page.authorizationGeneration !== user.authorizationGeneration || page.spaceId !== spaceId) {
    throw new Error("space journey administration receipt or session binding did not verify independently");
  }
  for (const forbidden of fixture.privacy.forbiddenPageSubstrings) {
    if (source.includes(`"${forbidden}"`)) throw new Error(`space journey administration page leaked the field '${forbidden}'`);
  }
  return { status: response.status, source, page };
}

type SpaceJourneyTraceEntry = { readonly kind: "step" | "skip"; readonly id: string; readonly actor: string; readonly outcome: string; readonly durationMs: number; readonly en: string; readonly de: string; readonly detail: Record<string, unknown> };

/** 🚻️ Drives the whole two-user collaboration journey against one real SQLite `os-hub` child through
 * the real authenticated routes only, then restarts the hub on the same data root. It claims no
 * document content edit, no rendered surface, no invite-token leg, and no presence roster. */
async function proveDirectorySpaceJourneyV1Process(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl,fixture:DirectorySpaceJourneyFixture): Promise<number> {
  return withPublishedLocalHubHostV1(invocation,phaseControl,async(host,context)=>{
  const {workspace:repoRoot,root,dataRoot}=context;
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot || !isAbsolute(artifactRoot)) throw new Error("space journey process requires an absolute ticket-local SEMIO_TEST_ARTIFACT_DIR");
  mkdirSync(artifactRoot, { recursive: true });
  const profiles: readonly LocalProfile[] = [
    { profileId: fixture.profiles.a.profileId, subject: fixture.profiles.a.subject, displayName: fixture.profiles.a.displayName, allowedClientClasses: ["native"] },
    { profileId: fixture.profiles.b.profileId, subject: fixture.profiles.b.subject, displayName: fixture.profiles.b.displayName, allowedClientClasses: ["native"] },
  ];
  const started = Date.now();
  const deadline = started + fixture.limits.journeyMs;
  const checkpoint = (): void => {
    if (Date.now() >= deadline) throw new Error("space journey deadline exceeded");
  };
  const trace: SpaceJourneyTraceEntry[] = [];
  const step = (id: string, at: number, outcome: string, detail: Record<string, unknown>): void => {
    const declared = spaceJourneyStep(fixture, id);
    trace.push({ kind: "step", id, actor: declared.actor, outcome, durationMs: Date.now() - at, en: declared.labels.en, de: declared.labels.de, detail });
  };
  const skip = (id: string, detail: Record<string, unknown>): void => {
    const declared = spaceJourneySkip(fixture, id);
    trace.push({ kind: "skip", id, actor: "none", outcome: "skipped", durationMs: 0, en: declared.labels.en, de: declared.labels.de, detail: { reason: declared.reason, ...detail } });
  };
  const submit = async (run: LocalHubRun, envelope: Record<string, any>, id: string, command: Record<string, unknown>, requestId = liveDirectoryCommandRequestId()): Promise<Record<string, any>> => {
    const at = Date.now();
    const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, command as unknown as DirectoryCommand));
    const { status, text } = await postLiveDirectoryCommand(run, envelope.capability, requestId, command, sealed);
    const receipt = await verifyLiveSpaceJourneyReceipt(text, status, requestId, command, spaceJourneyStep(fixture, id).expect);
    step(id, at, receipt.outcome as string, { status, events: receipt.events.length, commandSha256: receipt.commandSha256, requestBytes: Buffer.byteLength(sealed, "utf8") });
    return receipt;
  };

  let first: LocalHubRun | undefined;
  let second: LocalHubRun | undefined;
  let envelopeA: Record<string, any> | undefined;
  let envelopeB: Record<string, any> | undefined;
  let restartedA: Record<string, any> | undefined;
  let socketA: LiveDirectorySocketV1 | undefined;
  let socketB: LiveDirectorySocketV1 | undefined;
  let scopedB: LiveDirectorySocketV1 | undefined;
  try {
    first = await startLocalHub(host, profiles, { capture: true, isolatedSecuritySmoke: true });
    await waitForReadiness(first, true);
    envelopeA = await issueLocalCredential(first, profiles[0]!.profileId, "native", 2);
    envelopeB = await issueLocalCredential(first, profiles[1]!.profileId, "native", 3);
    const userA = await liveDirectoryEventPageUser(first, envelopeA);
    const userB = await liveDirectoryEventPageUser(first, envelopeB);
    if (typeof userB.email !== "string" || userB.email.length === 0 || userA.userId === userB.userId) throw new Error("space journey identities are not two distinct authenticated users");

    const createCommand = { kind: "create-space", name: fixture.space.name, spaceKind: fixture.space.spaceKind, visibility: fixture.space.visibility };
    const createRequestId = liveDirectoryCommandRequestId();
    const created = await submit(first, envelopeA, "create-space", createCommand, createRequestId);
    const spaceId = createdLiveDirectorySpace(created.events);

    checkpoint();
    await submit(first, envelopeA, "create-space-redacted-retry", createCommand, createRequestId);
    const afterRetry = await readLiveDirectoryHistory(first, envelopeA, userA, 0);
    const createdCount = afterRetry.events.filter((event) => event?.body?.kind === "space.created").length;
    if (createdCount !== 1) throw new Error(`space journey redacted retry minted ${createdCount} spaces`);

    let at = Date.now();
    const authorPage = await fetchLiveSpaceAdministrationPage(first, envelopeA, userA, spaceId, fixture);
    const authorExpect = spaceJourneyStep(fixture, "author-reads-administration-page").expect;
    const authorView = authorPage.page;
    if (!authorView || authorView.access !== authorExpect.access || !authorView.invites || !authorView.capabilities || authorView.members?.rows?.length !== authorExpect.memberRows) {
      throw new Error(`space journey author administration page differed: ${authorPage.status}/${authorPage.page?.access}`);
    }
    step("author-reads-administration-page", at, "author", { status: authorPage.status, bytes: Buffer.byteLength(authorPage.source, "utf8"), memberRows: authorView.members.rows.length, receiptVerified: true });

    at = Date.now();
    const hidden = await fetchLiveSpaceAdministrationPage(first, envelopeB, userB, spaceId, fixture);
    const hiddenExpect = spaceJourneyStep(fixture, "nonmember-administration-page-denied").expect;
    if (hidden.status !== hiddenExpect.status || (hiddenExpect.emptyBody && hidden.source.length !== 0)) throw new Error(`space journey non-member page read was not a non-enumerating denial: ${hidden.status}`);
    step("nonmember-administration-page-denied", at, String(hidden.status), { status: hidden.status, bodyBytes: hidden.source.length });

    checkpoint();
    await submit(first, envelopeA, "author-adds-b-as-spectator", { kind: "upsert-member", spaceId, email: userB.email, role: "spectator" });

    at = Date.now();
    const memberPage = await fetchLiveSpaceAdministrationPage(first, envelopeB, userB, spaceId, fixture);
    const memberExpect = spaceJourneyStep(fixture, "member-reads-administration-page").expect;
    const memberView = memberPage.page;
    if (!memberView || memberView.access !== memberExpect.access || "invites" in memberView || "capabilities" in memberView || memberView.members?.rows?.length !== memberExpect.memberRows) {
      throw new Error(`space journey member administration page differed: ${memberPage.status}/${memberPage.page?.access}`);
    }
    step("member-reads-administration-page", at, "member", { status: memberPage.status, bytes: Buffer.byteLength(memberPage.source, "utf8"), memberRows: memberView.members.rows.length, omitsInvites: true, omitsCapabilities: true });

    await submit(first, envelopeA, "author-promotes-b", { kind: "upsert-member", spaceId, email: userB.email, role: "author" });
    const descriptor = {
      spaceId,
      documentId: fixture.document.documentId,
      artifactKind: fixture.document.artifactKind,
      artifactSchema: fixture.document.artifactSchema,
      owner: fixture.document.owner,
      packSchemaHash: fixture.document.packSchemaHash,
      bootstrapVersion: fixture.document.bootstrapVersion,
      bootstrapFrontier: fixture.document.bootstrapFrontier,
      bootstrapSnapshotHash: fixture.document.bootstrapSnapshotHash,
    };
    await submit(first, envelopeA, "announce-document", { kind: "announce-document", descriptor });
    const privateB = await submit(first, envelopeB, "b-creates-private-space", { kind: "create-space", name: fixture.space.privateBName, spaceKind: fixture.space.spaceKind, visibility: "private" });
    const spaceB = createdLiveDirectorySpace(privateB.events);

    checkpoint();
    const openPlan = await fetch(`http://127.0.0.1:${first.port}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(fixture.document.documentId)}/open-plan`, {
      method: "POST",
      headers: { authorization: `Bearer ${envelopeA.capability}`, "content-type": "application/json" },
      body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId: fixture.document.documentId }, clientInstanceId: "space-journey-driver" }),
      signal: AbortSignal.timeout(5_000),
    });
    const presenceSkip = spaceJourneySkip(fixture, "document-open-plan-and-presence");
    if (!presenceSkip.admissibleStatuses.includes(openPlan.status)) throw new Error(`space journey document open plan answered ${openPlan.status}, which the honest skip does not admit`);
    skip("document-open-plan-and-presence", { openPlanStatus: openPlan.status, documentSocketDialled: false, presenceClaimed: false });
    skip("document-content-edit", { commandsIssued: 0 });
    skip("rendered-surface", { renderersDriven: 0 });
    skip("invite-token-leg", { invitesCreated: 0 });
    skip("alternate-directory-backends", { backend: "sqlite" });

    at = Date.now();
    const beforeWake = await readLiveDirectoryHistory(first, envelopeA, userA, 0);
    socketA = await openLiveDirectorySocket(first, envelopeA.capability, "/directory/socket-grants", `/directory/socket/v1?since=${beforeWake.frontier}`, fixture);
    socketB = await openLiveDirectorySocket(first, envelopeB.capability, "/directory/socket-grants", "/directory/socket/v1?since=0", fixture);
    step("open-global-sockets", at, "open", { protocol: fixture.bindings.socketProtocol, sinceA: beforeWake.frontier, sinceB: 0 });

    at = Date.now();
    const scopedPrefix = `/directory/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(fixture.document.documentId)}`;
    scopedB = await openLiveDirectorySocket(first, envelopeB.capability, `${scopedPrefix}/socket-grants`, `${scopedPrefix}/socket/v1?since=0`, fixture);
    step("open-scoped-socket", at, "open", { protocol: fixture.bindings.socketProtocol, since: 0 });

    checkpoint();
    const renamed = await submit(first, envelopeB, "member-renames-space", { kind: "rename-space", spaceId, name: fixture.space.renamedName });
    const renameSeq = renamed.events[0]!.seq as number;

    at = Date.now();
    const wakeSeen = () => socketA!.messages.filter((message) => message.kind === "event" && message.event?.seq === renameSeq).length;
    if (!(await waitForLiveDirectorySocket(socketA, () => wakeSeen() >= 1, fixture.limits.socketWaitMs))) throw new Error("space journey author never observed the peer rename as a live wake");
    await Bun.sleep(750);
    const wakes = wakeSeen();
    const socketSource = socketA.raw.join("\n");
    if (wakes !== spaceJourneyStep(fixture, "author-observes-dirty-wake").expect.sequences) throw new Error(`space journey author observed the peer rename ${wakes} times`);
    if (socketSource.includes(spaceB) || socketSource.includes(fixture.space.privateBName)) throw new Error("space journey author's live socket leaked a foreign private space");
    step("author-observes-dirty-wake", at, "wake", { sequences: wakes, frames: socketA.messages.length, foreignLeaks: 0 });

    at = Date.now();
    const history = await readLiveDirectoryHistory(first, envelopeA, userA, 0);
    let previous = 0;
    for (const event of history.events) {
      if (typeof event.seq !== "number" || event.seq <= previous) throw new Error("space journey visible history is not strictly increasing");
      previous = event.seq;
    }
    if (history.source.includes(spaceB) || history.source.includes(fixture.space.privateBName) || !history.source.includes(fixture.space.renamedName)) {
      throw new Error("space journey visible event page leaked a foreign private space or lost the peer rename");
    }
    const ids = history.events.map((event) => event.id ?? event.seq);
    if (new Set(ids).size !== ids.length) throw new Error("space journey visible history repeats an event");
    step("author-reads-ordered-event-page", at, "ordered", { events: history.events.length, frontier: history.frontier, leaks: 0, renameObserved: true });

    checkpoint();
    await submit(first, envelopeA, "author-removes-b", { kind: "remove-member", spaceId, userId: String(userB.userId) });

    at = Date.now();
    if (!(await waitForLiveDirectorySocket(scopedB, () => scopedB!.closeCode !== undefined, fixture.limits.socketWaitMs))) throw new Error("space journey scoped socket stayed live after the membership was removed");
    if (scopedB.closeCode !== spaceJourneyStep(fixture, "scoped-socket-revoked").expect.code) throw new Error(`space journey scoped socket closed with ${scopedB.closeCode}`);
    step("scoped-socket-revoked", at, String(scopedB.closeCode), {
      code: scopedB.closeCode,
      globalSocketCloseCode: socketB.closeCode ?? "open",
      note: "membership revocation is terminal for the scope-bound socket; the global session socket is bound to the session, not the membership",
    });

    at = Date.now();
    const removedRead = await fetchLiveSpaceAdministrationPage(first, envelopeB, userB, spaceId, fixture);
    const removedExpect = spaceJourneyStep(fixture, "removed-member-page-denied").expect;
    if (removedRead.status !== removedExpect.status || (removedExpect.emptyBody && removedRead.source.length !== 0)) throw new Error(`space journey removed member still read the page: ${removedRead.status}`);
    step("removed-member-page-denied", at, String(removedRead.status), { status: removedRead.status, bodyBytes: removedRead.source.length });

    at = Date.now();
    const revoked = await fetch(`http://127.0.0.1:${first.port}/auth/sessions/me/sign-out`, { method: "POST", headers: { authorization: `Bearer ${envelopeA.capability}` }, signal: AbortSignal.timeout(5_000) });
    const revokedBody = await revoked.arrayBuffer();
    if (revoked.status !== spaceJourneyStep(fixture, "author-self-revokes").expect.status || revokedBody.byteLength !== 0) throw new Error(`space journey self-revocation was not terminal: ${revoked.status}`);
    step("author-self-revokes", at, String(revoked.status), { status: revoked.status });

    at = Date.now();
    const stale = await fetch(`http://127.0.0.1:${first.port}/directory/event-page/v1?after=0`, { headers: { authorization: `Bearer ${envelopeA.capability}` }, signal: AbortSignal.timeout(5_000) });
    const staleBody = await stale.arrayBuffer();
    if (stale.status !== spaceJourneyStep(fixture, "self-revoked-read-denied").expect.status || staleBody.byteLength !== 0) throw new Error(`space journey stale bearer denial was not empty: ${stale.status}`);
    if (!(await waitForLiveDirectorySocket(socketA, () => socketA!.closeCode !== undefined, fixture.limits.socketWaitMs)) || socketA.closeCode !== 4401) {
      throw new Error(`space journey self-revoked author kept a live socket: ${socketA.closeCode}`);
    }
    step("self-revoked-read-denied", at, String(stale.status), { status: stale.status, bodyBytes: staleBody.byteLength, globalSocketCloseCode: socketA.closeCode });

    const priorBinding = String(authorView.sessionBindingSha256);
    closeLiveDirectorySocket(socketA);
    closeLiveDirectorySocket(socketB);
    closeLiveDirectorySocket(scopedB);
    envelopeA.capability = "";
    envelopeB.capability = "";
    await finishLocalHub(first);
    const firstExit = first.child.exitCode;
    first = undefined;

    at = Date.now();
    second = await startLocalHub(host, profiles, { capture: true, isolatedSecuritySmoke: true });
    await waitForReadiness(second, true);
    restartedA = await issueLocalCredential(second, profiles[0]!.profileId, "native", 2);
    const restartedUser = await liveDirectoryEventPageUser(second, restartedA);
    const restored = await fetchLiveSpaceAdministrationPage(second, restartedA, restartedUser, spaceId, fixture);
    const restoredExpect = spaceJourneyStep(fixture, "restart-preserves-history").expect;
    const restoredView = restored.page;
    if (!restoredView || restoredView.access !== restoredExpect.access || restoredView.members?.rows?.length !== restoredExpect.memberRows || restoredView.space?.name !== fixture.space.renamedName || restoredView.sessionBindingSha256 === priorBinding) {
      throw new Error(`space journey restart lost the space, its membership history, or rebound a stale session: ${restored.status}/${restored.page?.access}`);
    }
    const restoredHistory = await readLiveDirectoryHistory(second, restartedA, restartedUser, 0);
    const restoredSource = restoredHistory.source;
    if (!restoredSource.includes(fixture.space.renamedName) || !restoredSource.includes("member.removed") || restoredSource.includes(spaceB) || restoredSource.includes(fixture.space.privateBName)) {
      throw new Error("space journey restarted hub lost the durable membership history or leaked a foreign private space");
    }
    step("restart-preserves-history", at, "author", { status: restored.status, memberRows: restoredView.members.rows.length, name: restoredView.space.name, rebound: true, events: restoredHistory.events.length, priorHubExitCode: firstExit });

    await finishLocalHub(second);
    const secondExit = second.child.exitCode;
    second = undefined;
    const journeyMs = Date.now() - started;
    const binary = hubBinaryPath(repoRoot);
    const traceRecord = {
      schema: "semio.hub.directory-space-journey-trace/v1",
      fixture: fixture.schema,
      journeyMs,
      backend: "sqlite",
      binarySha256: existsSync(binary) ? createHash("sha256").update(readFileSync(binary)).digest("hex") : "unavailable",
      hubExitCodes: { first: firstExit, second: secondExit },
      steps: trace.filter((entry) => entry.kind === "step").length,
      skips: trace.filter((entry) => entry.kind === "skip").length,
      trace,
    };
    const traceJson = JSON.stringify(traceRecord, null, 2);
    const traceText = trace.map((entry) => `${entry.kind === "skip" ? "skip" : "step"} ${entry.id} actor=${entry.actor} outcome=${entry.outcome} ms=${entry.durationMs}\n  en: ${entry.en}\n  de: ${entry.de}`).join("\n");
    for (const forbidden of fixture.privacy.forbiddenTraceSubstrings) {
      if (traceJson.includes(forbidden) || traceText.includes(forbidden)) throw new Error(`space journey trace carried the forbidden substring '${forbidden}'`);
    }
    await Bun.write(join(artifactRoot, "space-journey-trace.json"), `${traceJson}\n`);
    await Bun.write(join(artifactRoot, "space-journey-trace.txt"), `${traceText}\n`);
    console.log(traceText);
    console.log(`space-journey-process: steps=${traceRecord.steps} skips=${traceRecord.skips} ms=${journeyMs} binarySha256=${traceRecord.binarySha256}`);
    rmSync(dataRoot, { recursive: true, force: true });
    return trace.length;
  } catch (error) {
    const diagnostics = `${first?.output() ?? ""}\n${second?.output() ?? ""}`.slice(-4_096);
    const partial = trace.map((entry) => `${entry.id}=${entry.outcome}`).join(" ");
    throw new Error(`${error instanceof Error ? error.message : "space journey process failed"}\ncompleted: ${partial}${diagnostics ? `\nhub diagnostics:\n${diagnostics}` : ""}`);
  } finally {
    closeLiveDirectorySocket(socketA);
    closeLiveDirectorySocket(socketB);
    closeLiveDirectorySocket(scopedB);
    if (envelopeA) envelopeA.capability = "";
    if (envelopeB) envelopeB.capability = "";
    if (restartedA) restartedA.capability = "";
    if (first) await finishLocalHub(first);
    if (second) await finishLocalHub(second);
  }
  });
}

export class SpaceJourneyCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = (segments[0] ?? "--source").replace(/^--/, "");
    if (segments.length > 1 || !["source", "process"].includes(phase)) throw new Error("space-journey-check accepts --source or --process");
    const { fixture, checks } = await proveDirectorySpaceJourneyV1(this.repoRoot);
    if (phase === "process") {
      runCmd("cargo", ["build", "--manifest-path", "Cargo.toml", "--bin", "os-hub", "--message-format=short"], { cwd: this.root, budgetMs: buildBudgetMs() });
      const observed = await proveDirectorySpaceJourneyV1Process(this.invocation,exactCargoCallerControlV1(this.invocation,"hub:space-journey",0),fixture);
      console.log(`space-journey-check: checks=${checks} phase=${phase} observed=${observed}`);
      return;
    }
    console.log(`space-journey-check: checks=${checks} phase=${phase} steps=${fixture.steps.length} skips=${fixture.skips.length}`);
  }
}
//#endregion 🔖️SpaceJourney
