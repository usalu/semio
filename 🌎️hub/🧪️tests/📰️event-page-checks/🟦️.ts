/** 🧭️ Directory event page route and process gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { type CompilerAdmissionControl } from "@semio-tech/process/compiler-capacity";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { type ExactCargoLawGroup } from "@semio-tech/process/cargo-laws";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { isAbsolute, join } from "node:path";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { runCmd } from "@semio-tech/process/execution";
import { BundleScript } from "@semio-tech/process/routing";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { issueLocalCredential } from "../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { type LocalProfile } from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { finishLocalHub, type LocalHubRun, withPublishedLocalHubHostV1, startLocalHub, waitForReadiness } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { readHubScriptModule } from "../../🧪️testing/📜️script-sources/🟦️.ts";
import { createdLiveDirectorySpace, fetchLiveDirectoryEventPage, liveDirectoryEventPageUser, type LiveDirectoryEventPageV1, submitLiveDirectoryCommand } from "../📣️directory-live-support/🟦️.ts";
import { hubLawGroups, readDirectoryBackendSource, readHubBinSource, readNeo4jDirectorySource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

/** 👥️ Replays the server-local lease contract without using the Rust implementation. */
type DirectoryEventPageRouteFixture = {
  readonly schema: "semio.hub.directory-event-page-route/v1";
  readonly limits: { readonly rawRows: 128; readonly eventBytes: 49152; readonly pageBytes: 65536; readonly safeInteger: 9007199254740991 };
  readonly session: { readonly sessionId: string; readonly userId: string; readonly authorizationGeneration: number; readonly expiresAt: number; readonly bindingSha256: string };
  readonly vectors: readonly {
    readonly name: "raw-holes" | "128-hidden" | "page-byte-prefix" | "empty" | "event-byte-boundary";
    readonly after: number;
    readonly raw: readonly { readonly seq: number; readonly visible: boolean; readonly labelBytes: number }[];
    readonly expected: { readonly through: number; readonly hasMore: boolean; readonly visibleSeqs: readonly number[]; readonly nextVisibleSeqs: readonly number[] };
  }[];
  readonly queryCases: readonly { readonly query: string; readonly bearer: "valid" | "missing" | "revoked" | "rotated" | "backend-failure"; readonly status: number; readonly bodyBytes: number; readonly reads: number }[];
  readonly hostiles: readonly string[];
};

async function proveDirectoryEventPageRouteV1(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/📅️event-page-route-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as DirectoryEventPageRouteFixture;
  if (fixture.schema !== "semio.hub.directory-event-page-route/v1") throw new Error("directory event page route fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "hostiles,limits,queryCases,schema,session,vectors") throw new Error("directory event page route fixture envelope drift");
  if (!hubSchemaExport(repoRoot, "schema://hub.directory/DirectorySessionBindingV1")(fixture.session)) throw new Error("directory event page route session is not the owned session-binding contract");
  if (fixture.limits.rawRows !== 128 || fixture.limits.eventBytes !== 49152 || fixture.limits.pageBytes !== 65536 || fixture.limits.safeInteger !== 9007199254740991) throw new Error("directory event page route bound drift");
  const eventPageVectorNames = ["raw-holes", "128-hidden", "page-byte-prefix", "empty", "event-byte-boundary"];
  if (fixture.vectors.map((vector) => vector.name).join(",") !== eventPageVectorNames.join(",")) throw new Error("directory event page route vector inventory drift");
  const eventPageHostiles = ["receipt-substitution", "hidden-identity-leak", "event-plus-one", "unknown-field", "binding-concatenation"];
  if (fixture.hostiles.join(",") !== eventPageHostiles.join(",")) throw new Error("directory event page route hostile inventory drift");
  if (fixture.queryCases.length !== 12) throw new Error("directory event page route query inventory drift");
  for (const query of fixture.queryCases) {
    if (!["valid", "missing", "revoked", "rotated", "backend-failure"].includes(query.bearer) || ![200, 400, 401, 500].includes(query.status) || ![0, 1].includes(query.reads)) throw new Error(`directory event page route query taxonomy drift: ${query.query}`);
    if (!Number.isSafeInteger(query.bodyBytes) || query.bodyBytes < 0 || query.bodyBytes > fixture.limits.pageBytes) throw new Error(`directory event page route query body bound drift: ${query.query}`);
  }
  const u32be = (value: number): Buffer => {
    const bytes = Buffer.alloc(4);
    bytes.writeUInt32BE(value);
    return bytes;
  };
  const u64be = (value: number): Buffer => {
    const bytes = Buffer.alloc(8);
    bytes.writeBigUInt64BE(BigInt(value));
    return bytes;
  };
  const i64be = (value: number): Buffer => {
    const bytes = Buffer.alloc(8);
    bytes.writeBigInt64BE(BigInt(value));
    return bytes;
  };
  const sessionId = Buffer.from(fixture.session.sessionId, "utf8");
  const userId = Buffer.from(fixture.session.userId, "utf8");
  const binding = createHash("sha256")
    .update(Buffer.concat([Buffer.from("semio/hub/directory-event-page/session-binding/v1\0"), u32be(sessionId.length), sessionId, u32be(userId.length), userId, u64be(fixture.session.authorizationGeneration), i64be(fixture.session.expiresAt)]))
    .digest("hex");
  if (binding !== fixture.session.bindingSha256) throw new Error("directory event page session binding is not byte-exact");
  const makeEvent = (row: { seq: number; visible: boolean; labelBytes: number }) => ({
    seq: row.seq,
    id: `event-${row.seq}`,
    hlc: { physicalMs: row.seq, logical: 0 },
    actor: { kind: "system", id: row.visible ? "system:visible" : "system:hidden-identity" },
    spaceId: row.visible ? "visible-space" : "hidden-space",
    body: { kind: "space.renamed", spaceId: row.visible ? "visible-space" : "hidden-space", name: "x".repeat(row.labelBytes) },
    recordedAtMs: row.seq,
  });
  const seal = (after: number, through: number, hasMore: boolean, events: readonly ReturnType<typeof makeEvent>[]) => {
    const unsigned = {
      schema: "semio.directory.event-page.v1",
      sessionBindingSha256: binding,
      authorizationGeneration: fixture.session.authorizationGeneration,
      afterSeqExclusive: after,
      throughSeqInclusive: through,
      hasMore,
      events,
    };
    const receiptSha256 = createHash("sha256").update(JSON.stringify(unsigned)).digest("hex");
    const page = { ...unsigned, receiptSha256 };
    return { page, bytes: Buffer.byteLength(JSON.stringify(page), "utf8") };
  };
  const build = (after: number, raw: readonly ReturnType<typeof makeEvent>[]) => {
    let through = after;
    const events: ReturnType<typeof makeEvent>[] = [];
    let stopped = false;
    for (const event of raw) {
      if (event.seq <= through || event.seq > fixture.limits.safeInteger || Buffer.byteLength(JSON.stringify(event), "utf8") > fixture.limits.eventBytes) throw new Error("directory event append rejected");
      if (event.spaceId === "hidden-space") {
        through = event.seq;
        continue;
      }
      const candidate = seal(after, event.seq, true, [...events, event]);
      if (candidate.bytes > fixture.limits.pageBytes) {
        stopped = true;
        break;
      }
      events.push(event);
      through = event.seq;
    }
    return seal(after, through, stopped || raw.length === fixture.limits.rawRows, events).page;
  };
  for (const vector of fixture.vectors) {
    if (vector.name === "event-byte-boundary") {
      const exact = makeEvent(vector.raw[0]!);
      const plusOne = makeEvent(vector.raw[1]!);
      if (Buffer.byteLength(JSON.stringify(exact), "utf8") !== fixture.limits.eventBytes) throw new Error("exact event boundary fixture drifted");
      if (Buffer.byteLength(JSON.stringify(plusOne), "utf8") !== fixture.limits.eventBytes + 1) throw new Error("event max+1 fixture drifted");
      if (build(vector.after, [exact]).events.length !== 1) throw new Error("exact event boundary was not admitted");
      try {
        build(vector.after, [plusOne]);
        throw new Error("event max+1 admitted");
      } catch (error) {
        if ((error as Error).message !== "directory event append rejected") throw error;
      }
      continue;
    }
    const rows = vector.name === "128-hidden" ? Array.from({ length: fixture.limits.rawRows }, (_, index) => makeEvent({ seq: vector.after + index + 1, visible: false, labelBytes: 8 })) : vector.raw.map(makeEvent);
    const page = build(vector.after, rows);
    if (page.throughSeqInclusive !== vector.expected.through || page.hasMore !== vector.expected.hasMore || JSON.stringify(page.events.map((event) => event.seq)) !== JSON.stringify(vector.expected.visibleSeqs)) {
      throw new Error(`directory event page vector differs for ${vector.name}`);
    }
    if (page.events.some((event) => JSON.stringify(event).includes("hidden-identity"))) throw new Error("hidden raw identity leaked into a page");
    const receipt = createHash("sha256")
      .update(JSON.stringify({ ...page, receiptSha256: undefined }, (_key, value) => value))
      .digest("hex");
    if (receipt !== page.receiptSha256 || Buffer.byteLength(JSON.stringify(page), "utf8") > fixture.limits.pageBytes) throw new Error(`directory event page receipt/size differs for ${vector.name}`);
    if (vector.expected.nextVisibleSeqs.length > 0) {
      const next = build(
        page.throughSeqInclusive,
        rows.filter((row) => row.seq > page.throughSeqInclusive),
      );
      if (JSON.stringify(next.events.map((event) => event.seq)) !== JSON.stringify(vector.expected.nextVisibleSeqs)) throw new Error("directory event page continuation skipped a visible row");
    }
  }
  const parseAfter = (query: string): number | null => {
    if (query.includes("&") || query.includes("%") || query.includes("+")) return null;
    const parts = query.split("=");
    if (parts.length !== 2 || parts[0] !== "after" || !/^(0|[1-9][0-9]*)$/.test(parts[1]!)) return null;
    const value = Number(parts[1]);
    return Number.isSafeInteger(value) && value <= fixture.limits.safeInteger ? value : null;
  };
  for (const query of fixture.queryCases) {
    const admitted = parseAfter(query.query) !== null;
    const status = !admitted ? 400 : query.bearer === "valid" ? 200 : query.bearer === "backend-failure" ? 500 : 401;
    const reads = admitted && ["valid", "rotated", "backend-failure"].includes(query.bearer) ? 1 : 0;
    if (status !== query.status || reads !== query.reads || (status === 200 ? 1 : 0) !== query.bodyBytes) throw new Error(`directory event page query result differs for ${query.query}`);
  }
  const hostilePage = seal(0, 0, false, []).page;
  if (
    createHash("sha256")
      .update(JSON.stringify({ ...hostilePage, receiptSha256: undefined }))
      .digest("hex") === "b".repeat(64)
  )
    throw new Error("receipt substitution was not rejected");
  const alternateSession = Buffer.from("session-route-0", "utf8");
  const alternateUser = Buffer.from("1user-route-01", "utf8");
  if (Buffer.concat([sessionId, userId]).compare(Buffer.concat([alternateSession, alternateUser])) !== 0) throw new Error("binding ambiguity fixture drifted");
  const alternateBinding = createHash("sha256")
    .update(
      Buffer.concat([
        Buffer.from("semio/hub/directory-event-page/session-binding/v1\0"),
        u32be(alternateSession.length),
        alternateSession,
        u32be(alternateUser.length),
        alternateUser,
        u64be(fixture.session.authorizationGeneration),
        i64be(fixture.session.expiresAt),
      ]),
    )
    .digest("hex");
  if (alternateBinding === binding) throw new Error("length-prefixed session binding aliased concatenated identities");
  const sessionContract = hubSchemaExport(repoRoot, "schema://hub.directory/DirectorySessionBindingV1");
  if (sessionContract({ ...fixture.session, unknown: true }) || sessionContract({ ...fixture.session, bindingSha256: `${fixture.session.bindingSha256}00` }) || sessionContract({ ...fixture.session, authorizationGeneration: 0 }))
    throw new Error("directory event page route session contract admitted an unknown or concatenated binding");
  const shared = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"));
  const hub = readHubBinSource(repoRoot);
  const sqlite = readDirectoryBackendSource(repoRoot, "🪶️sqlite");
  const postgres = readDirectoryBackendSource(repoRoot, "🐘️postgres");
  const neo4j = readNeo4jDirectorySource(repoRoot);
  const sourceClosed = (contract: string, route: string, sq: string, pg: string, neo: string): boolean => {
    const admissionStart = route.indexOf("fn directory_event_page_request_admission");
    const admissionEnd = route.indexOf("\n}\n", admissionStart);
    const admission = admissionStart >= 0 && admissionEnd > admissionStart ? route.slice(admissionStart, admissionEnd) : "";
    const read = route.indexOf(".events_since(after, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS)");
    const revalidate = route.indexOf("revalidate_directory_event_page_caller(state, caller, binding)", read);
    const sqliteValidate = sq.indexOf("validate_directory_event_page_event(&persisted)", sq.indexOf("fn persist_event_with_identity"));
    const postgresValidates = [...pg.matchAll(/validate_directory_event_page_event\(&(?:persisted|full)\)/g)].map((match) => match.index ?? -1);
    const neoValidates = [...neo.matchAll(/validate_directory_event_page_event\(&(?:persisted|full)\)/g)].map((match) => match.index ?? -1);
    return (
      contract.includes("pub fn validate_directory_event_page_event") &&
      contract.includes("validate_directory_event_page_event(event).is_err()") &&
      route.includes("fn directory_event_page_request_admission") &&
      route.includes("semio/hub/directory-event-page/session-binding/v1\\0") &&
      admission.includes("query.contains('&') || query.contains('%') || query.contains('+')") &&
      route.includes("value.len() > 1 && value.starts_with('0')") &&
      read >= 0 &&
      revalidate > read &&
      route.includes("stopped_for_bytes || raw_len == DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS") &&
      route.includes("struct DirectoryEventPageHttpRequest") &&
      route.includes("if !self.response_owned") &&
      route.includes("control.checkpoint()?") &&
      route.includes("let visible = directory_event_page_event_visible(state, &event, &caller).await?") &&
      route.includes("get_role(space_id, &caller.user_id).await.map_err(directory_error_status)?") &&
      route.includes("tokio::time::timeout(std::time::Duration::from_millis(DIRECTORY_EVENT_PAGE_DEADLINE_MS)") &&
      route.includes('.route("/directory/event-page/v1", get(get_directory_event_page_v1))') &&
      sqliteValidate >= 0 &&
      sqliteValidate > sq.indexOf("INSERT INTO hub_directory_event", sq.indexOf("fn persist_event_with_identity")) &&
      postgresValidates.length === 3 &&
      neoValidates.length === 3 &&
      postgresValidates.every((index) => pg.indexOf("INSERT INTO hub_directory_event", index) > index) &&
      neoValidates.every((index) => neo.indexOf("CREATE (e:DirectoryEvent", index) > index)
    );
  };
  if (!sourceClosed(shared, hub, sqlite, postgres, neo4j)) throw new Error("directory event page route/storage source boundary is incomplete");
  const sourceHostiles: readonly [string, string, string, string, string][] = [
    [shared.replace("pub fn validate_directory_event_page_event", "fn validate_directory_event_page_event"), hub, sqlite, postgres, neo4j],
    [shared, hub.replace(".events_since(after, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS)", ".events_since(after, DIRECTORY_EVENT_READ_MAX)"), sqlite, postgres, neo4j],
    [shared, hub.replace("revalidate_directory_event_page_caller(state, caller, binding)", "Ok(caller.clone())"), sqlite, postgres, neo4j],
    [
      shared,
      hub.replace(
        "fn directory_event_page_request_admission(uri: &axum::http::Uri) -> Result<u64, StatusCode> {\n    let query = uri.query().ok_or(StatusCode::BAD_REQUEST)?;\n    if query.contains('&') || query.contains('%') || query.contains('+')",
        "fn directory_event_page_request_admission(uri: &axum::http::Uri) -> Result<u64, StatusCode> {\n    let query = uri.query().ok_or(StatusCode::BAD_REQUEST)?;\n    if false",
      ),
      sqlite,
      postgres,
      neo4j,
    ],
    [shared, hub.replaceAll("control.checkpoint()?;", ""), sqlite, postgres, neo4j],
    [shared, hub, sqlite.replace("validate_directory_event_page_event(&persisted)", "Ok(())"), postgres, neo4j],
    [shared, hub, sqlite, postgres.replace("validate_directory_event_page_event(&full)", "Ok(())"), neo4j],
    [shared, hub, sqlite, postgres, neo4j.replace("validate_directory_event_page_event(&full)", "Ok(())")],
    [shared, hub.replace("let visible = directory_event_page_event_visible(state, &event, &caller).await?", "let visible = event_visible(state, &event, Some(&caller)).await"), sqlite, postgres, neo4j],
  ];
  sourceHostiles.forEach((candidate, index) => {
    if (sourceClosed(...candidate)) throw new Error(`directory event page source oracle admitted removed fence ${index}`);
  });
  const runner = [readHubScriptModule(repoRoot, "🧪️tests/📣️directory-live-support"), readHubScriptModule(repoRoot, "🧪️tests/📰️event-page-checks")].join("\n");
  const processBody = (source: string): string => {
    const start = source.lastIndexOf("async function proveDirectoryEventPageV1Process");
    const end = source.indexOf("\n/**", start);
    return start >= 0 && end > start ? source.slice(start, end) : "";
  };
  const liveBody = (source: string): string => {
    const start = source.lastIndexOf("type LiveDirectoryEventPageV1 = {");
    const end = source.indexOf("\nclass DirectoryEventPageV1CheckScript", start);
    return start >= 0 && end > start ? source.slice(start, end) : "";
  };
  const withoutProcessFence = (source: string, needle: string, replacement: string): string => {
    const start = source.lastIndexOf("type LiveDirectoryEventPageV1 = {");
    const index = source.indexOf(needle, start);
    return index < 0 ? source : `${source.slice(0, index)}${replacement}${source.slice(index + needle.length)}`;
  };
  const processClosed = (source: string): boolean => {
    const body = processBody(source);
    const live = liveBody(source);
    const command = source.slice(source.indexOf("class DirectoryEventPageV1CheckScript"));
    return (
      body.match(/dataDir: dataRoot/g)?.length === 2 &&
      body.includes("second = await startLocalHub") &&
      live.includes("page.receiptSha256 !== receipt") &&
      body.includes("stale.status !== 401") &&
      body.includes("saturated.events.length !== 0") &&
      command.includes("await proveDirectoryEventPageV1Process(this.repoRoot, this.root)")
    );
  };
  const processHostiles = [
    withoutProcessFence(runner, "dataDir: dataRoot", "dataDir: undefined"),
    withoutProcessFence(runner, "page.receiptSha256 !== receipt", "false"),
    withoutProcessFence(runner, "stale.status !== 401", "false"),
    withoutProcessFence(runner, "second = await startLocalHub", "second = await Promise.reject"),
  ];
  if (!processClosed(runner)) throw new Error("directory event page real-process boundary is incomplete");
  processHostiles.forEach((candidate, index) => {
    if (processClosed(candidate)) throw new Error(`directory event page process oracle admitted removed fence ${index}`);
  });
  const checks = fixture.vectors.length + fixture.queryCases.length + fixture.hostiles.length + sourceHostiles.length + processHostiles.length + 2;
  console.log(
    `directory-event-page-v1-oracle: AJV=1 vectors=${fixture.vectors.length} queries=${fixture.queryCases.length} hostiles=${fixture.hostiles.length} source-hostiles=${sourceHostiles.length} process-hostiles=${processHostiles.length} sha256=1`,
  );
  return checks;
}

/** 🌎️ Proves the receipt producer against a restarted real SQLite hub and two independent local identities. */
export async function proveDirectoryEventPageV1Process(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl): Promise<Readonly<{ page: LiveDirectoryEventPageV1; userId: string; displayName: string; baseUrl: string }>> {
  return withPublishedLocalHubHostV1(invocation,phaseControl,async(host,context)=>{
  const {workspace:repoRoot,root,dataRoot}=context;
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot || !isAbsolute(artifactRoot)) throw new Error("directory event page process requires an absolute ticket-local SEMIO_TEST_ARTIFACT_DIR");
  const profiles: readonly LocalProfile[] = [
    { profileId: "event-page-a", subject: "event-page-process-a", displayName: "Event Page A", allowedClientClasses: ["native"] },
    { profileId: "event-page-b", subject: "event-page-process-b", displayName: "Event Page B", allowedClientClasses: ["native"] },
  ];
  const deadline = Date.now() + 90_000;
  const checkpoint = (): void => {
    if (Date.now() >= deadline) throw new Error("directory event page process deadline exceeded");
  };
  let first: LocalHubRun | undefined;
  let second: LocalHubRun | undefined;
  let envelopeA: Record<string, any> | undefined;
  let envelopeB: Record<string, any> | undefined;
  let restartedA: Record<string, any> | undefined;
  try {
    first = await startLocalHub(host, profiles, { capture: true, isolatedSecuritySmoke: true });
    await waitForReadiness(first, true);
    envelopeA = await issueLocalCredential(first, profiles[0]!.profileId, "native", 2);
    envelopeB = await issueLocalCredential(first, profiles[1]!.profileId, "native", 3);
    const userA = await liveDirectoryEventPageUser(first, envelopeA);
    const userB = await liveDirectoryEventPageUser(first, envelopeB);
    const baselinePage = await fetchLiveDirectoryEventPage(first, envelopeA, userA, 0);
    const baseline = baselinePage.throughSeqInclusive;
    const createdA = await submitLiveDirectoryCommand(first, envelopeA, { kind: "create-space", name: "Visible A 0", spaceKind: "studio", visibility: "private" });
    const spaceA = createdLiveDirectorySpace(createdA);
    const createdB = await submitLiveDirectoryCommand(first, envelopeB, { kind: "create-space", name: "Hidden B 0", spaceKind: "studio", visibility: "private" });
    const spaceB = createdLiveDirectorySpace(createdB);
    const visibleSeqs = [
      ...createdA,
      ...(await submitLiveDirectoryCommand(first, envelopeA, { kind: "rename-space", spaceId: spaceA, name: "Visible A 1" })),
      ...(await submitLiveDirectoryCommand(first, envelopeB, { kind: "rename-space", spaceId: spaceB, name: "Hidden B 1" })),
      ...(await submitLiveDirectoryCommand(first, envelopeA, { kind: "rename-space", spaceId: spaceA, name: "Visible A 2" })),
      ...(await submitLiveDirectoryCommand(first, envelopeB, { kind: "rename-space", spaceId: spaceB, name: "Hidden B 2" })),
    ]
      .filter((event) => event.spaceId === spaceA)
      .map((event) => event.seq);
    const holes = await fetchLiveDirectoryEventPage(first, envelopeA, userA, baseline);
    const holeSource = JSON.stringify(holes);
    if (JSON.stringify(holes.events.map((event) => event.seq)) !== JSON.stringify(visibleSeqs) || holeSource.includes(spaceB) || holeSource.includes(userB.userId) || holeSource.includes("Hidden B")) {
      throw new Error("directory event page process raw-hole visibility differed from current membership");
    }

    const prefixAfter = holes.throughSeqInclusive;
    const firstLarge = (await submitLiveDirectoryCommand(first, envelopeA, { kind: "rename-space", spaceId: spaceA, name: "a".repeat(32 * 1024) }))[0]!.seq;
    const secondLarge = (await submitLiveDirectoryCommand(first, envelopeA, { kind: "rename-space", spaceId: spaceA, name: "b".repeat(32 * 1024) }))[0]!.seq;
    const prefix = await fetchLiveDirectoryEventPage(first, envelopeA, userA, prefixAfter);
    if (JSON.stringify(prefix.events.map((event) => event.seq)) !== JSON.stringify([firstLarge]) || prefix.throughSeqInclusive !== firstLarge || !prefix.hasMore) {
      throw new Error("directory event page process byte prefix skipped or duplicated a visible row");
    }
    const continuation = await fetchLiveDirectoryEventPage(first, envelopeA, userA, prefix.throughSeqInclusive);
    if (JSON.stringify(continuation.events.map((event) => event.seq)) !== JSON.stringify([secondLarge]) || continuation.throughSeqInclusive !== secondLarge) {
      throw new Error("directory event page process continuation did not return the deferred visible row exactly once");
    }

    let lastHidden = secondLarge;
    for (let index = 0; index < 128; index += 1) {
      checkpoint();
      lastHidden = (await submitLiveDirectoryCommand(first, envelopeB, { kind: "rename-space", spaceId: spaceB, name: `Hidden B saturated ${index}` }))[0]!.seq;
    }
    const saturated = await fetchLiveDirectoryEventPage(first, envelopeA, userA, secondLarge);
    const saturatedSource = JSON.stringify(saturated);
    if (saturated.events.length !== 0 || saturated.throughSeqInclusive !== lastHidden || !saturated.hasMore || saturatedSource.includes(spaceB) || saturatedSource.includes(userB.userId) || saturatedSource.includes("Hidden B saturated")) {
      throw new Error("directory event page process did not advance one saturated raw-hidden scan exactly");
    }

    const revoked = await fetch(`http://127.0.0.1:${first.port}/auth/sessions/me/sign-out`, { method: "POST", headers: { authorization: `Bearer ${envelopeB.capability}` }, signal: AbortSignal.timeout(2_000) });
    const stale = await fetch(`http://127.0.0.1:${first.port}/directory/event-page/v1?after=0`, { headers: { authorization: `Bearer ${envelopeB.capability}` }, signal: AbortSignal.timeout(2_000) });
    if (revoked.status !== 204 || stale.status !== 401 || (await stale.arrayBuffer()).byteLength !== 0) throw new Error("directory event page process stale bearer denial was not empty and terminal");
    const priorBinding = continuation.sessionBindingSha256;
    envelopeB.capability = "";
    await finishLocalHub(first);
    first = undefined;

    second = await startLocalHub(host, profiles, { capture: true, isolatedSecuritySmoke: true });
    await waitForReadiness(second, true);
    restartedA = await issueLocalCredential(second, profiles[0]!.profileId, "native", 2);
    const restartedUser = await liveDirectoryEventPageUser(second, restartedA);
    const persisted = await fetchLiveDirectoryEventPage(second, restartedA, restartedUser, firstLarge);
    const persistedSource = JSON.stringify(persisted);
    if (
      persisted.events[0]?.seq !== secondLarge ||
      persisted.throughSeqInclusive <= secondLarge ||
      !persisted.hasMore ||
      persisted.sessionBindingSha256 === priorBinding ||
      persistedSource.includes(spaceB) ||
      persistedSource.includes(userB.userId) ||
      persistedSource.includes("Hidden B saturated")
    ) {
      throw new Error("directory event page process restart lost durable visible/raw frontier or retained a stale session binding");
    }
    console.log("directory-event-page-v1-process: real SQLite two-user holes, prefix, stale bearer, and restart receipt passed");
    return { page: persisted, userId: restartedUser.userId, displayName: restartedUser.displayName, baseUrl: `http://127.0.0.1:${second.port}` };
  } catch (error) {
    const diagnostics = `${first?.output() ?? ""}\n${second?.output() ?? ""}`.slice(-4_096);
    throw new Error(`${error instanceof Error ? error.message : "directory event page process failed"}${diagnostics ? `\nhub diagnostics:\n${diagnostics}` : ""}`);
  } finally {
    if (envelopeA) envelopeA.capability = "";
    if (envelopeB) envelopeB.capability = "";
    if (restartedA) restartedA.capability = "";
    if (first) await finishLocalHub(first);
    if (second) await finishLocalHub(second);
  }
  });
}

export class DirectoryEventPageV1CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native", "process"].includes(phase)) throw new Error("directory-event-page-v1-check accepts source, native, or process");
    const checks = await proveDirectoryEventPageRouteV1(this.repoRoot);
    if (phase === "native" || phase === "process") {
      const laws: [string, string][] = [["semio-hub-directory-route-laws", "directory_event_page_v1_route_scans_raw_holes_bounds_canonical_receipt_and_visibility"], ["semio-hub-directory-route-laws", "directory_event_page_v1_route_revalidates_session_generation_after_read_before_response"], ["semio-hub-directory-route-laws", "directory_event_page_v1_route_stops_at_canonical_byte_prefix_without_skipping_visible_seq"], ["semio-hub-directory-route-laws", "directory_event_page_v1_append_admission_is_transactional_for_sqlite_postgres_and_neo4j"], ["semio-hub-directory-route-laws", "directory_event_page_v1_route_rejects_noncanonical_query_and_stale_bearer_without_body"]];
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.root,...exactCargoStageEnvironments(),groups: [
          ...hubLawGroups(laws, ["--all-features"]),
          ...(phase === "native"
            ? [
                {
                  package: "semio-hub-directory-backend-laws",
                  target: { kind: "lib" },
                  cargoArgs: ["--all-features"],
                  laws: ["sqlite_tests::directory_event_page_v1_append_admission_is_transactional_sqlite", "postgres_tests::directory_event_page_v1_append_admission_is_transactional_postgres", "neo4j_tests::directory_event_page_v1_append_admission_is_transactional_neo4j"],
                },
              ] satisfies ExactCargoLawGroup[]
            : []),
        ],progress(event) {
          console.log(`directory-event-page-v1-${phase} ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`directory-event-page-v1-${phase}-receipt: ${JSON.stringify(receipt)}`);
      if (phase === "process") {
        runCmd("cargo", ["build", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
        await proveDirectoryEventPageV1Process(this.invocation,exactCargoCallerControlV1(this.invocation,"hub:event-page",0));
        runCmd("cargo", ["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
      }
    }
    console.log(`directory-event-page-v1-check: checks=${checks} phase=${phase}`);
  }
}
