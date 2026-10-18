/** 🧭️ The check-in process gate: genesis, edit commits and heads over a real MCP workspace child.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { withTrustedCatalogCleanupV1 } from "../../🏗️bootstrap/🧾️provenance/🧹️settlement/🟦️.ts";
import { repositoryCargoNetworkV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { verifyCapturedCargoArtifactsV1 } from "@semio-tech/process/native-build";
import { readNativeCallerAuthorityV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { type CompilerAdmissionControl } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { parseSpaceArtifactCreationStatusJsonV1 } from "@semio-tech/framework-os/modules/directory/io/text/space-artifact-creation-v1";
import { withExactCargoLawOutputV1 } from "@semio-tech/process/testing/cargo/exact";
import { withCheckInProcessFixtureV1 } from "../../🏗️bootstrap/🧾️provenance/📦️cargo/🧫️fixture/📥️reading/🟦️.ts";
import type { CheckInProcessFixtureDataV1 } from "../../🏗️bootstrap/🧾️provenance/📦️cargo/🧫️fixture/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { createHash, randomBytes } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { type ChildProcess } from "node:child_process";
import { withRepositoryPublishedCargoExecutableV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/📥️published/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { decodeServerFrame, encodeClientFrame, type WireFrontierSummary, type WireMutationEnvelope } from "@semio-tech/framework-replication";
import { parseDocumentSocketGrantReceiptV1 } from "@semio-tech/framework-os";
import { documentCheckInCanonicalJson, parseDocumentCheckInStatusV1, parseDocumentCheckInV1 } from "@semio-tech/framework-os/modules/directory/io/text/document-check-in-v1";
import { type DocumentCheckInStatusV1 } from "@semio-tech/framework/directory/schema";
import { parseDocumentOpenPlanV1, parseDocumentPlanSocketGrantIntentV1 } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { sealSpaceArtifactCreateV1 } from "@semio-tech/framework-os/modules/directory/schema/space-artifact-creation-v1";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { DIRECT_CHILD_BENIGN_ENV_KEY, DIRECT_CHILD_BENIGN_ENV_VALUE } from "../../🔐️auth/📤️credential-delivery/🟦️.ts";
import { issueLocalCredential } from "../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { type LocalProfile } from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { finishLocalHub, type LocalHubRun, type LocalHubHostV1, withLocalHubHostTaskV1, withLocalHubRunTaskV1, withBuiltLocalHubHostV1, startLocalHub, waitForChildExit, waitForReadiness } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { type CheckInProcessEnvelopeV1, type CheckInProcessFixtureV1, liveDirectoryCommandRequestId, postLiveDirectoryCommand, startMcpBoundWorkspaceChild, waitForDocumentSocketFrame, waitForJsonRpcResponse } from "../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";
import { hubLawGroups, readHubBinSource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

/** 🌱️ Creates the initial canonical pair through the retained server-owned creation transaction. */
export async function createCheckInProcessGenesis(run: LocalHubRun, capability: string, spaceId: string, fixture: CheckInProcessFixtureV1): Promise<CheckInProcessFixtureV1> {
  const request = sealSpaceArtifactCreateV1({
    requestId: randomBytes(16).toString("hex"),
    expectedCatalogGenerationId: fixture.generationId,
    kindId: fixture.artifact.kind,
    name: "Verified GIS Map",
  });
  const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const accepted = await fetch(`http://127.0.0.1:${run.port}${route}`, {
    method: "POST",
    headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" },
    body: JSON.stringify(request),
    signal: AbortSignal.timeout(5_000),
  });
  let status = parseSpaceArtifactCreationStatusJsonV1(await accepted.text());
  if (
    ![200, 202].includes(accepted.status) ||
    status.requestId !== request.requestId ||
    status.spaceId !== spaceId ||
    status.catalogGenerationId !== request.expectedCatalogGenerationId
  ) throw new Error(`check-in process genesis was not durably accepted: ${accepted.status}`);
  const deadline = Date.now() + 120_000;
  while (status.phase !== "ready") {
    if (!["accepted", "preparing", "indeterminate"].includes(status.phase) || Date.now() >= deadline) throw new Error(`check-in process genesis reached ${status.phase} before Ready`);
    await Bun.sleep(10);
    const response = await fetch(`http://127.0.0.1:${run.port}${route}/${encodeURIComponent(request.requestId)}`, { headers: { authorization: `Bearer ${capability}` }, signal: AbortSignal.timeout(5_000) });
    status = parseSpaceArtifactCreationStatusJsonV1(await response.text());
    if (response.status !== 200 && response.status !== 202) throw new Error(`check-in process genesis status failed: ${response.status}`);
    if (status.catalogGenerationId !== request.expectedCatalogGenerationId) throw new Error("check-in process genesis status crossed its selected catalog generation");
  }
  if (
    status.ready?.kindId !== fixture.artifact.kind ||
    status.ready.artifactSchema !== fixture.artifact.schema ||
    status.ready.parentDialect.artifactKind !== fixture.artifact.kind ||
    !/^artifact-[0-9a-f]{32}$/u.test(status.ready.artifactId)
  ) throw new Error("check-in process genesis Ready coordinates differ from the selected GIS target");
  return Object.freeze({ ...fixture, documentId: status.ready.artifactId });
}

/** ✍️ Commits the fixture's real GIS Map ledger edit through an authenticated document socket and
 * returns the committed head the hub acknowledged — the head a Check In then names. */
export async function commitCheckInProcessEdit(
  run: LocalHubRun,
  capability: string,
  spaceId: string,
  fixture: CheckInProcessFixtureV1,
  envelopes: readonly CheckInProcessEnvelopeV1[],
): Promise<{ readonly plan: ReturnType<typeof parseDocumentOpenPlanV1>; readonly frontier: WireFrontierSummary }> {
  const documentRoot = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(fixture.documentId)}`;
  const planResponse = await fetch(`http://127.0.0.1:${run.port}${documentRoot}/open-plan`, {
    method: "POST",
    headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" },
    body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId: fixture.documentId }, requestedSurfaceId: fixture.surfaceId, clientInstanceId: "check-in-process" }),
    signal: AbortSignal.timeout(5_000),
  });
  const plan = parseDocumentOpenPlanV1(await planResponse.json().catch(() => undefined), Date.now());
  if (!planResponse.ok || plan.scope.spaceId !== spaceId || plan.scope.documentId !== fixture.documentId || plan.catalog.generationId !== fixture.generationId) throw new Error("check-in process received a substituted GIS Map plan");
  const grantIntent = parseDocumentPlanSocketGrantIntentV1({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt });
  const grantResponse = await fetch(`http://127.0.0.1:${run.port}${documentRoot}/socket-grants`, {
    method: "POST",
    headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" },
    body: JSON.stringify(grantIntent),
    signal: AbortSignal.timeout(5_000),
  });
  const grant = parseDocumentSocketGrantReceiptV1(await grantResponse.json().catch(() => undefined));
  if (!grantResponse.ok) throw new Error(`check-in process socket grant failed: ${grantResponse.status}`);
  const scope = `${spaceId}/${fixture.documentId}`;
  const socket = new WebSocket(`ws://127.0.0.1:${run.port}/scopes/${encodeURIComponent(scope)}/document/ws?surface=${encodeURIComponent(fixture.surfaceId)}`, ["semio.session.v1", capability]);
  socket.binaryType = "arraybuffer";
  const frames: Record<string, any>[] = [];
  let socketError: unknown;
  socket.onmessage = (event) => {
    try {
      frames.push(decodeServerFrame(new Uint8Array(event.data as ArrayBuffer)).frame as unknown as Record<string, any>);
    } catch (error) {
      socketError = error;
    }
  };
  socket.onerror = (event) => {
    socketError = event;
  };
  try {
    await new Promise<void>((resolveOpen, rejectOpen) => {
      const timer = setTimeout(() => rejectOpen(new Error("check-in process socket open deadline exceeded")), 5_000);
      socket.onopen = () => {
        clearTimeout(timer);
        resolveOpen();
      };
    });
    if (socket.protocol !== grant.protocol) throw new Error("check-in process socket did not negotiate its exact protocol");
    const packSchemaHash = Buffer.from(fixture.artifact.packSchemaHash, "hex");
    socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: fixture.artifact.schema, pack_schema_hash: Array.from(packSchemaHash), resume_token: null, frontier: null } }, "command"));
    await waitForDocumentSocketFrame(socket, frames, (frame) => ("Welcome" in frame ? frame.Welcome : undefined), "Welcome");
    await waitForDocumentSocketFrame(socket, frames, (frame) => ("Session" in frame && frame.Session.actor === grant.actorId ? frame.Session : undefined), "verified Session actor");
    if (socketError) throw socketError;
    const documentId = `v1:${Buffer.byteLength(spaceId)}:${Buffer.byteLength(fixture.documentId)}:${spaceId}${fixture.documentId}`;
    const physicalMs = Date.now();
    const wire: WireMutationEnvelope[] = envelopes.map((envelope, index) => ({
      mutation_id: envelope.mutationId,
      document_id: documentId,
      actor: grant.actorId,
      dependencies: [...envelope.dependencies],
      observed: null,
      target: [],
      diff: { schema: envelope.diffSchema, payload: Array.from(Buffer.from(envelope.diff, "base64")) },
      inverse: { schema: envelope.inverseSchema, payload: Array.from(Buffer.from(envelope.inverse, "base64")) },
      timestamp: { actor: 1, physical_ms: physicalMs, logical: index + 1 },
      transaction: null,
      verb: null,
      line: null,
    }));
    socket.send(encodeClientFrame({ Commands: { batch_id: 1, envelopes: wire } }, "command"));
    const ack = await waitForDocumentSocketFrame(socket, frames, (frame) => ("Ack" in frame && frame.Ack.batch_id === 1 ? frame.Ack : undefined), "persisted command acknowledgement");
    const accepted = ack.stages.some((stage: any) => stage === "Persisted") && ack.stages.some((stage: any) => stage?.Applied?.outcome === "Accepted");
    const tip = envelopes[envelopes.length - 1]!.mutationId;
    if (!accepted || ack.frontier.head_edit_id !== tip || ack.frontier.head_edit_ordinal !== envelopes.length || ack.frontier.last_commit_seq !== 1)
      throw new Error("check-in process edit was not durably accepted at its exact first frontier");
    return { plan, frontier: ack.frontier as WireFrontierSummary };
  } finally {
    if (socket.readyState < WebSocket.CLOSING) socket.close(1000, "edit committed");
  }
}

/** 📌️ Checks in one acknowledged head through the hub's Check In command and follows its status to
 * `ready`, then proves a lost-response retry of the same request answers the identical checkpoint. */
export async function checkInProcessHeadV1(run: LocalHubRun, capability: string, spaceId: string, documentId: string, frontier: WireFrontierSummary): Promise<DocumentCheckInStatusV1> {
  const chainSha256 = Buffer.from(frontier.chain_hash).toString("hex");
  const request = parseDocumentCheckInV1(
    documentCheckInCanonicalJson({
      schema: "semio.hub.document-check-in/v1",
      requestId: randomBytes(16).toString("hex"),
      head: { documentId, headEditOrdinal: frontier.head_edit_ordinal, headEditId: frontier.head_edit_id, lastCommitSeq: frontier.last_commit_seq, chainSha256 },
    }),
  );
  if (!request) throw new Error("check-in process could not seal its canonical request");
  const route = `http://127.0.0.1:${run.port}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/check-ins`;
  const post = async (): Promise<DocumentCheckInStatusV1> => {
    const response = await fetch(route, { method: "POST", headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" }, body: documentCheckInCanonicalJson(request), signal: AbortSignal.timeout(10_000) });
    const status = parseDocumentCheckInStatusV1(await response.text());
    if (![200, 202].includes(response.status) || !status || status.requestId !== request.requestId) throw new Error(`check-in process command was not accepted: ${response.status}`);
    return status;
  };
  let status = await post();
  const deadline = Date.now() + 120_000;
  while (status.phase !== "ready") {
    if (status.phase === "failed" || status.phase === "cancelled" || Date.now() >= deadline) throw new Error(`check-in process ended ${status.phase}${status.refusal ? ` (${status.refusal})` : ""}`);
    await Bun.sleep(20);
    const response = await fetch(`${route}/${request.requestId}`, { headers: { authorization: `Bearer ${capability}` }, signal: AbortSignal.timeout(5_000) });
    const next = parseDocumentCheckInStatusV1(await response.text());
    if (![200, 202].includes(response.status) || !next || next.requestId !== request.requestId || next.progress.completedUnits < status.progress.completedUnits) throw new Error(`check-in process status regressed: ${response.status}`);
    status = next;
  }
  if (JSON.stringify(status.ready?.baseline) !== JSON.stringify(request.head)) throw new Error("check-in process checkpoint baseline is not the named head");
  const replay = await post();
  if (replay.phase !== "ready" || JSON.stringify(replay.ready) !== JSON.stringify(status.ready)) throw new Error("check-in lost-response retry did not answer its identical checkpoint");
  return status;
}

async function proveCheckInMcpProcess(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,data:CheckInProcessFixtureDataV1,mcpBinary:string):Promise<void>{
 return withLocalHubHostTaskV1(host,phaseControl,async context=>{
 const {workspace:repoRoot}=context,{fixture:loadedFixture,envelopes}=data;
  let fixture:CheckInProcessFixtureV1 = loadedFixture;
  const profile: LocalProfile = { profileId: "check-in-process", subject: "check-in-process-author", displayName: "Check In Author", allowedClientClasses: ["native", "mcp"] };
  const run = await startLocalHub(host,[profile],{capture:true,isolatedSecuritySmoke:true});
  let child: ChildProcess | undefined;
  return withTrustedCatalogCleanupV1(()=>withLocalHubRunTaskV1(run,async()=>{
  try {
    const readiness = await waitForReadiness(run);
    if (readiness.artifactAuthority?.ready !== true || readiness.features?.openPlan !== true || readiness.features?.openPlanExchange !== true) throw new Error("checkpoint publication process Hub did not load its verified GIS Map authority");
    const author = await issueLocalCredential(run, profile.profileId, "native", 6);
    const created = await postLiveDirectoryCommand(run, author.capability, liveDirectoryCommandRequestId(), { kind: "create-space", name: "MCP Cold GIS Map", spaceKind: "studio", visibility: "private" });
    const createdBody = created.status === 202 ? (JSON.parse(created.text) as Record<string, any>) : undefined;
    const spaceId = createdBody?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
    if (typeof spaceId !== "string" || spaceId.length === 0) throw new Error("checkpoint publication process could not create its private space");
    fixture = await createCheckInProcessGenesis(run, author.capability, spaceId, fixture);
    const { plan, frontier } = await commitCheckInProcessEdit(run, author.capability, spaceId, fixture, envelopes);
    const checkedIn = await checkInProcessHeadV1(run, author.capability, spaceId, fixture.documentId, frontier);

    const otherCreated = await postLiveDirectoryCommand(run, author.capability, liveDirectoryCommandRequestId(), { kind: "create-space", name: "MCP Cross-Scope GIS Map", spaceKind: "studio", visibility: "private" });
    const otherBody = otherCreated.status === 202 ? (JSON.parse(otherCreated.text) as Record<string, any>) : undefined;
    const otherSpaceId = otherBody?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
    if (typeof otherSpaceId !== "string" || otherSpaceId.length === 0) throw new Error("checkpoint publication process could not create its cross-scope space");
    const otherFixture = await createCheckInProcessGenesis(run, author.capability, otherSpaceId, fixture);
    if (otherFixture.documentId === fixture.documentId) throw new Error("checkpoint publication process creation reused a server-minted document id across spaces");

    const mcpEnvelope = await issueLocalCredential(run, profile.profileId, "mcp", 2);
    const protectedValues = ["poison-user", "poison-session", "poison-token", "poison-origin", "poison-auth", "poison-cookie"];
    child = await startMcpBoundWorkspaceChild(repoRoot, run, mcpEnvelope, spaceId, {
      ...context.environment,SEMIO_OS_MCP_BIN:mcpBinary,
      [DIRECT_CHILD_BENIGN_ENV_KEY]: DIRECT_CHILD_BENIGN_ENV_VALUE,
      S_USER: protectedValues[0],
      S_SESSION: protectedValues[1],
      NPM_TOKEN: protectedValues[2],
      S_HUB_URL: protectedValues[3],
      AUTHORIZATION: protectedValues[4],
      COOKIE: protectedValues[5],
    });
    const stdout: Buffer[] = [];
    const stderr: Buffer[] = [];
    let stdoutBytes = 0;
    let stderrBytes = 0;
    child.stdout?.on("data", (chunk: Buffer) => {
      stdoutBytes += chunk.byteLength;
      if (stdoutBytes <= 32_768) stdout.push(Buffer.from(chunk));
    });
    child.stderr?.on("data", (chunk: Buffer) => {
      stderrBytes += chunk.byteLength;
      if (stderrBytes <= 16_384) stderr.push(Buffer.from(chunk));
    });
    const writeRequest = (id: number, method: string, params: Record<string, unknown>): void => {
      child?.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
    };
    try {
      const boundDeadline = Date.now() + 10_000;
      while (!Buffer.concat(stderr).toString("utf8").includes("real per-capability ArtifactChannel routing bound")) {
        if (child.exitCode !== null) throw new Error(`checkpoint publication MCP child exited before workspace binding: ${child.exitCode}`);
        if (Date.now() >= boundDeadline) throw new Error("checkpoint publication MCP child workspace binding deadline exceeded");
        await Bun.sleep(20);
      }
      writeRequest(1, "initialize", { protocolVersion: "2025-11-25", capabilities: {}, clientInfo: { name: "semio-check-in-oracle", version: "1" } });
      const initialized = await waitForJsonRpcResponse(child, stdout, 1);
      if (initialized.result?.protocolVersion !== "2025-11-25") throw new Error("checkpoint publication MCP child initialization failed");
      child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
      writeRequest(2, "resources/list", {});
      const listed = await waitForJsonRpcResponse(child, stdout, 2);
      const checkpointUri = `semio://workspace/scopes/${encodeURIComponent(spaceId)}/${encodeURIComponent(fixture.documentId)}/checkpoint`;
      const descriptorUri = `semio://workspace/scopes/${encodeURIComponent(spaceId)}/${encodeURIComponent(fixture.documentId)}/descriptor`;
      if (!Array.isArray(listed.result?.resources) || !listed.result.resources.some((resource: any) => resource?.uri === checkpointUri) || !listed.result.resources.some((resource: any) => resource?.uri === descriptorUri)) {
        throw new Error("checkpoint publication MCP resource list omitted its exact scoped GIS Map resources");
      }
      writeRequest(3, "resources/read", { uri: checkpointUri });
      const read = await waitForJsonRpcResponse(child, stdout, 3);
      const text = read.result?.contents?.[0]?.text;
      const resource = typeof text === "string" ? (JSON.parse(text) as Record<string, any>) : undefined;
      const decodedPack = typeof resource?.untrusted?.content?.packBase64 === "string" ? Buffer.from(resource.untrusted.content.packBase64, "base64") : Buffer.alloc(0);
      const decodedSpr = typeof resource?.untrusted?.content?.sprBase64 === "string" ? Buffer.from(resource.untrusted.content.sprBase64, "base64") : Buffer.alloc(0);
      if (
        resource?.schema !== "semio.mcp.canonical-checkpoint-resource/v1" ||
        resource?.scope?.spaceId !== spaceId ||
        resource?.scope?.documentId !== fixture.documentId ||
        resource?.descriptorDigestV1 !== plan.descriptorDigestV1 ||
        resource?.activeCheckpointId !== checkedIn.ready?.checkpointId ||
        resource?.frontier?.headEditOrdinal !== checkedIn.ready?.baseline.headEditOrdinal ||
        resource?.frontier?.headEditId !== checkedIn.ready?.baseline.headEditId ||
        resource?.frontier?.lastCommitSeq !== checkedIn.ready?.baseline.lastCommitSeq ||
        decodedPack.byteLength === 0 ||
        decodedSpr.byteLength === 0 ||
        createHash("sha256").update(decodedPack).digest("hex") !== resource?.pack?.sha256 ||
        createHash("sha256").update(decodedSpr).digest("hex") !== resource?.spr?.sha256
      )
        throw new Error("check-in MCP resource did not project the hub-materialized GIS Map checkpoint");
      decodedPack.fill(0);
      decodedSpr.fill(0);

      const crossUri = `semio://workspace/scopes/${encodeURIComponent(otherSpaceId)}/${encodeURIComponent(fixture.documentId)}/checkpoint`;
      writeRequest(4, "resources/read", { uri: crossUri });
      const cross = await waitForJsonRpcResponse(child, stdout, 4);
      if (!cross.error || cross.result?.contents !== undefined) throw new Error("checkpoint publication MCP resource crossed its authenticated space boundary");

      const changedDescriptor = {
        spaceId,
        documentId: fixture.documentId,
        artifactKind: fixture.artifact.kind,
        artifactSchema: fixture.artifact.schema,
        owner: { pluginId: fixture.package.pluginId, packageId: fixture.package.packageId, version: fixture.package.version, packageHash: fixture.package.componentSha256 },
        packSchemaHash: fixture.artifact.packSchemaHash,
        bootstrapVersion: 2,
        bootstrapFrontier: { headSeq: 0, commitSeq: 0, epoch: 0 },
        bootstrapSnapshotHash: "44".repeat(32),
      };
      const changed = await postLiveDirectoryCommand(run, author.capability, liveDirectoryCommandRequestId(), { kind: "announce-document", descriptor: changedDescriptor });
      if (changed.status !== 409) throw new Error(`checkpoint publication process mutable descriptor was not refused: ${changed.status}`);
      writeRequest(10, "resources/read", { uri: descriptorUri });
      const retainedDescriptor = await waitForJsonRpcResponse(child, stdout, 10);
      const retainedDescriptorText = retainedDescriptor.result?.contents?.[0]?.text;
      if (typeof retainedDescriptorText !== "string" || JSON.parse(retainedDescriptorText)?.view?.descriptor?.bootstrapVersion !== 1) {
        throw new Error("checkpoint publication MCP descriptor changed after immutable replacement refusal");
      }
      writeRequest(11, "resources/read", { uri: checkpointUri });
      const retainedCheckpoint = await waitForJsonRpcResponse(child, stdout, 11);
      if (retainedCheckpoint.error || typeof retainedCheckpoint.result?.contents?.[0]?.text !== "string") throw new Error("checkpoint publication MCP resource lost its canonical pair after immutable descriptor refusal");
      child.stdin?.end();
      await waitForChildExit(child, 5_000);
      const output = Buffer.concat(stdout);
      const diagnostics = Buffer.concat(stderr).toString("utf8");
      if (
        child.exitCode !== 0 ||
        stdoutBytes !== output.byteLength ||
        stdoutBytes > 32_768 ||
        stderrBytes > 16_384 ||
        output.includes(Buffer.from(mcpEnvelope.capability)) ||
        diagnostics.includes(mcpEnvelope.capability) ||
        protectedValues.some((value) => output.includes(Buffer.from(value)) || diagnostics.includes(value))
      ) {
        throw new Error(`checkpoint publication MCP process byte-clean law failed: exit=${child.exitCode} stdout=${stdoutBytes} stderr=${stderrBytes}`);
      }
    } finally {
      stdout.forEach((bytes) => bytes.fill(0));
      stderr.forEach((bytes) => bytes.fill(0));
    }
  } finally {
    if (child) terminateOwnedChildTree(child);
  }
 }),[()=>finishLocalHub(run)]);
 });
}

/** 📌️ Check In contract oracle: every language-neutral vector through Ajv (third party) and the
 * TypeScript twin, then the source boundary the command depends on end to end. */
async function proveDocumentCheckInV1(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/📌️document-check-in-v1/🔣️.json"), "utf8"));
  const validateRequest = hubSchemaExport(repoRoot, "schema://os.directory/DocumentCheckInV1");
  const validateStatus = hubSchemaExport(repoRoot, "schema://os.directory/DocumentCheckInStatusV1");
  let checks = 0;
  const parseJson = (source: string): unknown => {
    try {
      return JSON.parse(source);
    } catch {
      return undefined;
    }
  };
  for (const row of fixture.valid.requests) {
    if (!validateRequest(parseJson(row.source)) || parseDocumentCheckInV1(row.source) === null) throw new Error(`check-in valid request refused: ${row.name}`);
    checks++;
  }
  for (const row of fixture.valid.statuses) {
    if (!validateStatus(parseJson(row.source)) || parseDocumentCheckInStatusV1(row.source) === null) throw new Error(`check-in valid status refused: ${row.name}`);
    checks++;
  }
  for (const row of fixture.invalid.requests) {
    if (Boolean(validateRequest(parseJson(row.source))) !== row.schemaValid || parseDocumentCheckInV1(row.source) !== null) throw new Error(`check-in invalid request differs: ${row.name}`);
    checks++;
  }
  for (const row of fixture.invalid.statuses) {
    if (Boolean(validateStatus(parseJson(row.source))) !== row.schemaValid || parseDocumentCheckInStatusV1(row.source) !== null) throw new Error(`check-in invalid status differs: ${row.name}`);
    checks++;
  }
  const hub = readHubBinSource(repoRoot);
  const job = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/📌️check-in/🦀️.rs"), "utf8");
  const ledger = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/⚙️engine/🦀️.rs"));
  const store = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"));
  const codec = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🦀️.rs"), "utf8");
  const wit = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️codec.wit"), "utf8");
  for (const [name, body, markers] of [
    ["hub", hub, ["/spaces/{space_id}/documents/{document_id}/check-ins", "/spaces/{space_id}/documents/{document_id}/check-ins/{request_id}/cancel", "artifact_ledger_tail", "claim_or_read_checkpoint_publication"]],
    ["job", job, ["TrustedArtifactReplayCodec", "materialize_check_in", "DocumentCheckInRefusalV1"]],
    ["ledger", ledger, ["pub async fn artifact_ledger_tail"]],
    ["store", store, ["ingest_remote"]],
    ["codec", codec, ["pub async fn replay_envelopes_onto_pair"]],
    ["wit", wit, ["replay-envelopes: async func"]],
  ] as const) {
    for (const marker of markers) if (!body.includes(marker)) throw new Error(`check-in ${name} boundary is missing ${marker}`);
    checks++;
  }
  if (hub.includes("checkpoint-publications")) throw new Error("the client upload route must stay deleted");
  console.log(`document-check-in-oracle: requests=${fixture.valid.requests.length}+${fixture.invalid.requests.length} statuses=${fixture.valid.statuses.length}+${fixture.invalid.statuses.length} ajv=2 typescript=2 boundaries=6`);
  return checks;
}

/** 📌️ `check-in-check source|native|process`: contract oracle, in-process hub laws, and the MCP process. */
export class CheckInCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native", "process"].includes(phase)) throw new Error("check-in-check accepts source, native, or process");
    const checks = await proveDocumentCheckInV1(this.repoRoot);
    if (phase === "native" || phase === "process") {
      const laws: [string, string][] = [["semio-hub-hostile-input-laws", "check_in_advances_the_active_checkpoint_to_the_named_head_and_cold_opens_from_it"], ["semio-hub-hostile-input-laws", "check_in_refuses_stale_unknown_and_foreign_heads_and_is_author_owned"], ["semio-hub-hostile-input-laws", "check_in_is_idempotent_per_request_cancellable_and_revoked_with_the_author"]];
      if (phase === "process") laws.push(["semio-hub-boot-laws", "check_in_process_fixture_emits_verified_gis_ledger_and_catalog"]);
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 180_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
          ...hubLawGroups(laws, ["--features", "integration-fixtures"]),
        ],consume:async outputs=>{
         if(phase!=="process")return;
         await withExactCargoLawOutputV1(outputs,{version:1,receiptIndex:outputs.receipts.findIndex(row=>row.package==="semio-hub-boot-laws")},output=>withBuiltLocalHubHostV1(this.invocation,output.control,(host,context)=>withCheckInProcessFixtureV1(host,context.control,outputs,{version:1,purpose:"runtime"},async(data,actual)=>{
          const manifest=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml");
          await buildRepositoryCargoArtifacts(manifest,["--bins"],this.repoRoot,{producer:"artifact",invocation:this.invocation,control:actual,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),consume:verifyCapturedCargoArtifactsV1});
          await withRepositoryPublishedCargoExecutableV1(manifest,process.platform==="win32"?"semio-os-mcp.exe":"semio-os-mcp",this.repoRoot,readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode,this.invocation,actual,(mcp,mcpPhase)=>proveCheckInMcpProcess(host,mcpPhase,data,mcp.path));
         })));
        },progress(event) {
          console.log(`check-in-${phase} ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`check-in-${phase}-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`check-in-check: checks=${checks} phase=${phase}`);
  }
}
