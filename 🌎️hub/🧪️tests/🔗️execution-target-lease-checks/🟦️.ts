/** 🧭️ Execution target lease corpus and the lease gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { createHash, webcrypto } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { decodePackValue, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, parseDocumentOpenPlanV1, parseDocumentExecutionTargetLeaseFieldsV1, leaseFieldsFromPlanV1, sameLeaseFieldsV1 } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "@semio-tech/framework/directory/schema";
import { runCargo } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts";
import { runCmd, orchestratorBudgetOpts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { runProbe } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔍️probe/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { documentOpenNeutralBrowserActor } from "../🗺️open-plan-checks/🟦️.ts";
import { readHubBinSource, readTrustedCatalogSource } from "../../🧪️testing/📖️source-readers/🟦️.ts";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

type ExecutionTargetLeaseFixture = {
  readonly schema: string;
  readonly version: 1;
  readonly nowMs: number;
  readonly hubOrigin: string;
  readonly intent: Record<string, any>;
  readonly plan: Record<string, any>;
  readonly manifest: Record<string, any>;
  readonly socketGrant: Record<string, any>;
  readonly componentHex: string;
  readonly descriptorHex: string;
  readonly expected: Record<string, any>;
  readonly hostile: readonly { readonly name: string; readonly stage: string; readonly kind: string; readonly path?: string; readonly value?: unknown; readonly expected: "unpublished" }[];
};

/** 🪪️ Independent hand-written full-field lease relation. It imports no production comparison and is
 * the corpus oracle's only admission decision. */
function executionTargetLeaseFieldsEqual(left: Record<string, any>, right: Record<string, any>): boolean {
  const scalarPaths = [
    "schema",
    "version",
    "scope.spaceId",
    "scope.documentId",
    "descriptorDigestV1",
    "catalog.generationId",
    "package.pluginId",
    "package.packageId",
    "package.version",
    "package.componentSha256",
    "package.componentBlake3",
    "package.descriptorByteSha256",
    "package.executionProtocol.appChannelVersion",
    "component.sha256",
    "component.blake3",
    "component.byteLength",
    "descriptor.sha256",
    "descriptor.byteLength",
    "browserActor.kind",
    "browserActor.schema",
    "browserActor.codegenPolicy",
    "browserActor.sha256",
    "browserActor.sourceComponentSha256",
    "browserActor.sourceDescriptorByteSha256",
    "browserActor.policySha256",
    "browserActor.byteLength",
    "artifact.kind",
    "artifact.schema",
    "artifact.packSchemaHash",
    "parentDialect.artifactKind",
    "parentDialect.standard",
    "parentDialect.subset",
    "surface.surfaceId",
    "surface.appId",
    "surface.windowKindId",
    "surface.role",
    "surface.rendererTarget",
    "grant.read",
    "grant.write",
    "grant.observe",
    "checkpoint.checkpointId",
    "checkpoint.descriptorDigestV1",
    "checkpoint.aggregateSha256",
    "checkpoint.baselineFrontier.documentId",
    "checkpoint.baselineFrontier.headEditOrdinal",
    "checkpoint.baselineFrontier.headEditId",
    "checkpoint.baselineFrontier.lastCommitSeq",
    "revalidation.directoryRevision",
    "revalidation.membershipGeneration",
    "revalidation.sessionGeneration",
    "revalidation.shareGeneration",
  ];
  const read = (source: Record<string, any>, path: string): unknown => path.split(".").reduce<any>((cursor, segment) => (cursor === undefined || cursor === null ? undefined : cursor[segment]), source);
  return (
    scalarPaths.every((path) => read(left, path) === read(right, path)) &&
    JSON.stringify(read(left, "browserActor.importInterfaces")) === JSON.stringify(read(right, "browserActor.importInterfaces")) &&
    JSON.stringify(read(left, "checkpoint.baselineFrontier.chainHash")) === JSON.stringify(read(right, "checkpoint.baselineFrontier.chainHash"))
  );
}

/** 🪪️ Independent structural admission of one lease-fields value: every identity, byte bound and
 * grant/role invariant, decided without importing the production parser. */
export function executionTargetLeaseFieldsAdmissible(candidate: Record<string, any>, expected: Record<string, any>): boolean {
  try {
    documentOpenNeutralBrowserActor(candidate.browserActor, candidate.package, candidate.surface?.rendererTarget, true);
  } catch {
    return false;
  }
  const hash = (value: unknown): boolean => typeof value === "string" && /^[0-9a-f]{64}$/u.test(value) && !/^0{64}$/u.test(value);
  const text = (value: unknown): boolean => typeof value === "string" && value.length > 0 && Buffer.byteLength(value, "utf8") <= 256 && ![...value].some((character) => character.codePointAt(0)! < 0x20 || character.codePointAt(0)! === 0x7f);
  const length = (value: unknown, maximum: number): boolean => typeof value === "number" && Number.isSafeInteger(value) && value >= 1 && value <= maximum;
  const generation = (value: unknown): boolean => typeof value === "number" && Number.isSafeInteger(value) && value >= 1 && value <= 9_007_199_254_740_991;
  return (
    candidate.schema === "semio.os.document-execution-target-lease/v1" &&
    candidate.version === 1 &&
    [
      candidate.scope?.spaceId,
      candidate.scope?.documentId,
      candidate.package?.pluginId,
      candidate.package?.packageId,
      candidate.package?.version,
      candidate.artifact?.kind,
      candidate.artifact?.schema,
      candidate.surface?.surfaceId,
      candidate.surface?.appId,
      candidate.surface?.windowKindId,
    ].every(text) &&
    [
      candidate.descriptorDigestV1,
      candidate.catalog?.generationId,
      candidate.package?.componentSha256,
      candidate.package?.componentBlake3,
      candidate.package?.descriptorByteSha256,
      candidate.artifact?.packSchemaHash,
      candidate.component?.sha256,
      candidate.component?.blake3,
      candidate.descriptor?.sha256,
    ].every(hash) &&
    candidate.component.sha256 === candidate.package.componentSha256 &&
    candidate.component.blake3 === candidate.package.componentBlake3 &&
    candidate.descriptor.sha256 === candidate.package.descriptorByteSha256 &&
    candidate.package.executionProtocol?.appChannelVersion === DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 &&
    length(candidate.component?.byteLength, expected.componentMaxBytes) &&
    length(candidate.descriptor?.byteLength, expected.descriptorMaxBytes) &&
    candidate.parentDialect?.artifactKind === candidate.artifact?.kind &&
    [candidate.parentDialect?.artifactKind, candidate.parentDialect?.standard, candidate.parentDialect?.subset].every((value) => text(value) && String(value).trim() === value) &&
    candidate.grant?.read === true &&
    candidate.grant?.observe === true &&
    typeof candidate.grant?.write === "boolean" &&
    ["viewer", "editor"].includes(candidate.surface?.role) &&
    ["react", "wgpu", "wasm"].includes(candidate.surface?.rendererTarget) &&
    candidate.grant.write === (candidate.surface.role === "editor") &&
    hash(candidate.checkpoint?.checkpointId) && candidate.checkpoint?.descriptorDigestV1 === candidate.descriptorDigestV1 && hash(candidate.checkpoint?.aggregateSha256) &&
    generation(candidate.revalidation?.directoryRevision) &&
    generation(candidate.revalidation?.membershipGeneration) &&
    (candidate.revalidation?.sessionGeneration === undefined) !== (candidate.revalidation?.shareGeneration === undefined)
  );
}

function executionTargetLeaseMutate(source: Record<string, any>, path: string, value: unknown): Record<string, any> {
  const candidate = structuredClone(source);
  const segments = path.split(".");
  let cursor: any = candidate;
  for (const segment of segments.slice(0, -1)) cursor = cursor[segment];
  cursor[segments.at(-1)!] = value;
  return candidate;
}

/** 🤖️ Independent Node state machine for one browser install: it walks manifest → component →
 * descriptor → verify → exchange with its own byte reader and hashers, and answers `published` only
 * when every field and byte agrees. It never imports the browser worker. */
type ExecutionTargetLeaseInstallInput = { manifest: Record<string, any>; component: Buffer; declaredComponentLength: number; descriptor: Buffer; declaredDescriptorLength: number; planGeneration: string; cancelAt?: string; missing?: string };

function executionTargetLeaseInstall(
  fixture: ExecutionTargetLeaseFixture,
  input: ExecutionTargetLeaseInstallInput,
  blake3Hex: (bytes: Uint8Array) => string,
): { readonly outcome: "published" | "unpublished"; readonly stage: string } {
  const stages = ["manifest", "component", "descriptor", "verify", "exchange"] as const;
  const cancel = input.cancelAt;
  for (const stage of stages) {
    if (cancel === stage) return { outcome: "unpublished", stage };
    if (input.missing === stage) return { outcome: "unpublished", stage };
    if (stage === "manifest") {
      if (!executionTargetLeaseFieldsAdmissible(input.manifest, fixture.expected)) return { outcome: "unpublished", stage };
      const projection = { ...structuredClone(fixture.plan), catalog: { generationId: input.planGeneration } } as Record<string, any>;
      const planFields = {
        schema: "semio.os.document-execution-target-lease/v1",
        version: 1,
        scope: projection.scope,
        descriptorDigestV1: projection.descriptorDigestV1,
        catalog: projection.catalog,
        package: projection.package,
        component: { sha256: projection.package.componentSha256, blake3: projection.package.componentBlake3, byteLength: input.manifest.component?.byteLength },
        descriptor: { sha256: projection.package.descriptorByteSha256, byteLength: input.manifest.descriptor?.byteLength },
        browserActor: projection.browserActor.kind === "none" ? projection.browserActor : { ...projection.browserActor, byteLength: input.manifest.browserActor?.byteLength },
        artifact: projection.artifact,
        parentDialect: projection.parentDialect,
        surface: projection.surface,
        grant: projection.grant,
        checkpoint: projection.checkpoint,
        revalidation: projection.revalidation,
      };
      if (!executionTargetLeaseFieldsEqual(planFields, input.manifest)) return { outcome: "unpublished", stage };
      continue;
    }
    if (stage === "component") {
      if (input.declaredComponentLength !== input.manifest.component.byteLength || input.declaredComponentLength > fixture.expected.componentMaxBytes || input.component.length !== input.declaredComponentLength)
        return { outcome: "unpublished", stage };
      continue;
    }
    if (stage === "descriptor") {
      if (input.declaredDescriptorLength !== input.manifest.descriptor.byteLength || input.declaredDescriptorLength > fixture.expected.descriptorMaxBytes || input.descriptor.length !== input.declaredDescriptorLength)
        return { outcome: "unpublished", stage };
      continue;
    }
    if (stage === "verify") {
      const componentSha256 = createHash("sha256").update(input.component).digest("hex");
      const descriptorSha256 = createHash("sha256").update(input.descriptor).digest("hex");
      if (componentSha256 !== input.manifest.component.sha256 || blake3Hex(input.component) !== input.manifest.component.blake3 || descriptorSha256 !== input.manifest.descriptor.sha256) return { outcome: "unpublished", stage };
      continue;
    }
    if (input.planGeneration !== input.manifest.catalog.generationId) return { outcome: "unpublished", stage };
  }
  return { outcome: "published", stage: "exchange" };
}

export async function proveExecutionTargetLeaseCorpus(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as ExecutionTargetLeaseFixture;
  const validateFields = hubSchemaExport(repoRoot, "schema://os.directory/DocumentExecutionTargetLeaseFieldsV1");
  if (fixture.schema !== "semio.os.document-execution-target-lease-corpus/v1" || fixture.version !== 1) throw new Error("execution target lease corpus schema drift");
  if (Object.keys(fixture).sort().join(",") !== "componentHex,componentReceiptCases,descriptorHex,expected,hostile,hubOrigin,intent,manifest,nowMs,plan,schema,socketGrant,version") throw new Error("execution target lease corpus envelope drift");
  if (!Number.isSafeInteger(fixture.nowMs) || fixture.nowMs < 1 || !/^https?:\/\/[^/]+$/u.test(fixture.hubOrigin)) throw new Error("execution target lease corpus clock/origin drift");
  if (!/^(?:[0-9a-f]{2})+$/u.test(fixture.componentHex) || !/^(?:[0-9a-f]{2})+$/u.test(fixture.descriptorHex)) throw new Error("execution target lease corpus asset bytes are not canonical hex");
  if (!hubSchemaExport(repoRoot, "schema://hub.directory/DocumentOpenIntentV1")(fixture.intent)) throw new Error("execution target lease corpus intent is not the owned document-open-intent contract");
  if (!hubSchemaExport(repoRoot, "schema://os.directory/DocumentSocketGrantReceiptV1")(fixture.socketGrant)) throw new Error("execution target lease corpus socket grant is not the owned document-socket-grant contract");
  const leaseStatusKeys = ["verifying", "retrying", "catching-up", "integrity-failed", "stale", "cancelled", "renderer-unavailable", "link-expired", "access-revoked"];
  const localizedText = hubSchemaExport(repoRoot, "schema://hub.directory/LocalizedTextV1");
  const expected = fixture.expected as Record<string, any>;
  if (Object.keys(expected).sort().join(",") !== "assetBody,assetMethod,assetPaths,componentMaxBytes,descriptorMaxBytes,forbiddenStatusFragments,manifestMaxBytes,openPlanPath,progressStages,progressUnitBytes,rendererClaims,rendererState,rotation,socketGrantPath,status,statusRoles,viewerWriteRejectedLocally")
    throw new Error("execution target lease corpus expectation envelope drift");
  if (expected.assetMethod !== "POST" || expected.assetBody !== "semio.hub.document-open-intent/v1" || expected.componentMaxBytes !== DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES || expected.descriptorMaxBytes !== DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES || expected.manifestMaxBytes !== 8192 || expected.progressUnitBytes !== 65536)
    throw new Error("execution target lease corpus bound drift");
  if (expected.progressStages.join(",") !== "manifest,component,descriptor,verify" || expected.rendererState !== "renderer-unavailable" || expected.viewerWriteRejectedLocally !== true) throw new Error("execution target lease corpus staging drift");
  if (Object.values(expected.rendererClaims as Record<string, unknown>).some((claim) => claim !== false) || Object.keys(expected.rendererClaims).sort().join(",") !== "activationRegistry,attachBackbone,loadPluginModule,wgpuScanner") throw new Error("execution target lease corpus renderer claims drift");
  if (expected.assetPaths.length !== 3 || expected.assetPaths.some((path: string, index: number) => path !== `${expected.openPlanPath.slice(0, expected.openPlanPath.lastIndexOf("/"))}/execution-target/${["manifest", "component", "descriptor"][index]}`)) throw new Error("execution target lease corpus asset path drift");
  if (Object.keys(expected.status).sort().join(",") !== leaseStatusKeys.slice().sort().join(",") || Object.keys(expected.statusRoles).sort().join(",") !== leaseStatusKeys.slice().sort().join(",")) throw new Error("execution target lease corpus status inventory drift");
  for (const key of leaseStatusKeys) {
    if (!localizedText(expected.status[key])) throw new Error(`execution target lease corpus status ${key} is not the owned bilingual text contract`);
    if (!["status", "alert"].includes(expected.statusRoles[key])) throw new Error(`execution target lease corpus status role drift: ${key}`);
  }
  if (!/^(?!0{64}$)[0-9a-f]{64}$/u.test(expected.rotation.generationA) || !/^(?!0{64}$)[0-9a-f]{64}$/u.test(expected.rotation.generationB) || expected.rotation.generationA === expected.rotation.generationB) throw new Error("execution target lease corpus rotation drift");
  if (fixture.hostile.length < 40 || new Set(fixture.hostile.map((row) => row.name)).size !== fixture.hostile.length) throw new Error("execution target lease corpus hostile inventory drift");
  for (const row of fixture.hostile) {
    if (!["manifest", "component", "descriptor", "verify", "exchange", "publication", "asset-request"].includes(row.stage)) throw new Error(`execution target lease corpus hostile stage drift: ${row.name}`);
  }
  const shippedReceipt = hubSchemaExport(repoRoot, "schema://os.directory/DocumentShippedComponentReceiptV1");
  for (const row of (fixture as unknown as { componentReceiptCases: readonly { id: string; path: readonly string[] | null; value: unknown; accepted: boolean }[] }).componentReceiptCases) {
    const lease = structuredClone(fixture.manifest) as Record<string, any>;
    if (row.path === null) delete lease.package.componentReceipt;
    else if (row.path.length === 1) lease.package.componentReceipt[row.path[0]!] = row.value;
    let admitted = true;
    try {
      parseDocumentExecutionTargetLeaseFieldsV1(lease);
    } catch {
      admitted = false;
    }
    if (admitted !== row.accepted || (admitted && (!validateFields(lease) || !shippedReceipt(lease.package.componentReceipt)))) throw new Error(`execution target lease corpus receipt law drift: ${row.id}`);
  }
  for (const [name, row] of [
    ["DocumentOpenPlanV1", fixture.plan],
    ["DocumentExecutionTargetLeaseFieldsV1", fixture.manifest],
  ] as const) {
    const shape = hubSchemaExport(repoRoot, `schema://os.directory/${name}`);
    if (!shape(row)) throw new Error(`canonical actor contract ${name} rejects its own corpus member`);
    const absent = structuredClone(row);
    delete absent.browserActor;
    if (shape(absent)) throw new Error(`canonical contract allows absent actor: ${name}`);
  }
  const { blake3Hex } = await import(join(repoRoot, "🧰️framework/🔨️modules/🔏️hash/🟦️.ts"));
  const component = Buffer.from(fixture.componentHex, "hex");
  const descriptor = Buffer.from(fixture.descriptorHex, "hex");
  if (component.length === 0 || descriptor.length === 0) throw new Error("execution target lease corpus must pin non-empty component and descriptor bytes");
  const componentSha256 = createHash("sha256").update(component).digest("hex");
  const descriptorSha256 = createHash("sha256").update(descriptor).digest("hex");
  const componentBlake3 = blake3Hex(component);
  const webComponentSha256 = Buffer.from(await webcrypto.subtle.digest("SHA-256", component)).toString("hex");
  if (componentSha256 !== webComponentSha256) throw new Error("execution target lease corpus Node and WebCrypto SHA-256 disagree");
  if (blake3Hex(Buffer.from("abc", "utf8")) !== "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85") throw new Error("first-party BLAKE3 known-answer vector regressed");
  if (componentSha256 !== fixture.manifest.component.sha256 || componentBlake3 !== fixture.manifest.component.blake3 || descriptorSha256 !== fixture.manifest.descriptor.sha256)
    throw new Error("execution target lease corpus digests do not match its exact bytes");
  if (component.length !== fixture.manifest.component.byteLength || descriptor.length !== fixture.manifest.descriptor.byteLength) throw new Error("execution target lease corpus byte lengths drifted");
  if (fixture.plan.surface.rendererTarget !== "wasm" || fixture.manifest.surface.role !== "viewer" || fixture.manifest.grant.write !== false) throw new Error("execution target lease corpus lost its read-only GIS Map wasm viewer positive");

  const parsedPlan = parseDocumentOpenPlanV1(fixture.plan, fixture.nowMs);
  const parsedManifest = parseDocumentExecutionTargetLeaseFieldsV1(fixture.manifest);
  const projected = leaseFieldsFromPlanV1(parsedPlan, { component: component.length, descriptor: descriptor.length, browserActor: fixture.manifest.browserActor.byteLength });
  if (!sameLeaseFieldsV1(projected, parsedManifest)) throw new Error("execution target lease production projection differs");
  for (const field of ["browserActor", "browserActor.sha256", "browserActor.policySha256", "browserActor.sourceComponentSha256", "browserActor.sourceDescriptorByteSha256", "browserActor.importInterfaces"]) {
    const plan = structuredClone(fixture.plan);
    const parts = field.split(".");
    if (parts.length === 1) delete plan.browserActor;
    else delete plan.browserActor[parts[1]!];
    let denied = false;
    try {
      parseDocumentOpenPlanV1(plan, fixture.nowMs);
    } catch {
      denied = true;
    }
    if (!denied) throw new Error(`document plan admitted absent actor identity: ${field}`);
  }
  const generationA = fixture.expected.rotation.generationA;
  const baseline = { manifest: fixture.manifest as Record<string, any>, component, declaredComponentLength: component.length, descriptor, declaredDescriptorLength: descriptor.length, planGeneration: generationA };
  const positive = executionTargetLeaseInstall(fixture, baseline, blake3Hex);
  if (positive.outcome !== "published") throw new Error(`execution target lease corpus positive vector was denied at ${positive.stage}`);

  let manifestFields = 0;
  let byteVectors = 0;
  let lifecycleVectors = 0;
  for (const vector of fixture.hostile) {
    if (vector.expected !== "unpublished") throw new Error(`execution target lease corpus hostile ${vector.name} is not an unpublished expectation`);
    if (vector.kind === "manifest-field") {
      const mutated = executionTargetLeaseMutate(fixture.manifest, vector.path!, vector.value);
      const admitted = validateFields(mutated) && executionTargetLeaseInstall(fixture, { ...baseline, manifest: mutated }, blake3Hex).outcome === "published";
      if (admitted) throw new Error(`execution target lease corpus admitted single-field substitution ${vector.name}`);
      let productionAdmitted = false;
      try {
        productionAdmitted = sameLeaseFieldsV1(projected, parseDocumentExecutionTargetLeaseFieldsV1(mutated));
      } catch {}
      if (productionAdmitted) throw new Error(`execution target lease production admitted substitution ${vector.name}`);
      manifestFields += 1;
      continue;
    }
    if (
      vector.kind === "cancel" ||
      vector.kind === "deadline" ||
      vector.kind === "reconnect-after-invalidation" ||
      vector.kind === "viewer-write" ||
      vector.kind === "caller-url" ||
      vector.kind === "caller-path" ||
      vector.kind === "caller-module" ||
      vector.kind === "stale-plan"
    ) {
      if (vector.kind === "cancel" || vector.kind === "deadline") {
        const stage = vector.kind === "deadline" ? "component" : vector.stage;
        if (executionTargetLeaseInstall(fixture, { ...baseline, cancelAt: stage }, blake3Hex).outcome === "published") throw new Error(`execution target lease corpus published through ${vector.name}`);
      }
      if (vector.kind === "stale-plan" && executionTargetLeaseInstall(fixture, { ...baseline, planGeneration: fixture.expected.rotation.generationB }, blake3Hex).outcome === "published")
        throw new Error("execution target lease corpus exchanged a stale rotated plan");
      if (vector.kind === "viewer-write" && fixture.expected.viewerWriteRejectedLocally !== true) throw new Error("execution target lease corpus lost its local viewer write rejection");
      if ((vector.kind === "caller-url" || vector.kind === "caller-path" || vector.kind === "caller-module") && fixture.expected.assetPaths.includes(String(vector.value)))
        throw new Error(`execution target lease corpus accepted caller substitution ${vector.name}`);
      lifecycleVectors += 1;
      continue;
    }
    if (vector.kind === "descriptor-protocol") {
      const decoded = packValueToExactJson(decodePackValue(descriptor)) as Record<string, unknown>;
      decoded.executionProtocol = { appChannelVersion: vector.value };
      const protocol = (packValueToExactJson(decodePackValue(encodePackValue(decoded))) as Record<string, any>).executionProtocol;
      if (protocol?.appChannelVersion === DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1) throw new Error(`execution target lease corpus hostile ${vector.name} did not change the compiled protocol`);
      byteVectors += 1;
      continue;
    }
    const mutated: ExecutionTargetLeaseInstallInput = { ...baseline };
    if (vector.kind === "component-bytes" || vector.kind === "mixed-generation") mutated.component = Buffer.concat([Buffer.from([component[0]! ^ 0xff]), component.subarray(1)]);
    else if (vector.kind === "component-truncated") mutated.component = component.subarray(0, component.length - 1);
    else if (vector.kind === "component-extra-byte") mutated.component = Buffer.concat([component, Buffer.from([7])]);
    else if (vector.kind === "component-max-plus-one") mutated.declaredComponentLength = Number(vector.value);
    else if (vector.kind === "descriptor-bytes" || vector.kind === "descriptor-self-hash" || vector.kind === "descriptor-noncanonical")
      mutated.descriptor = Buffer.concat([descriptor.subarray(0, descriptor.length - 1), Buffer.from([descriptor.at(-1)! ^ 0xff])]);
    else if (vector.kind === "descriptor-trailing-byte") mutated.descriptor = Buffer.concat([descriptor, Buffer.from([0])]);
    else if (vector.kind === "descriptor-max-plus-one") mutated.declaredDescriptorLength = Number(vector.value);
    else if (vector.kind === "missing-body") mutated.missing = vector.stage;
    else throw new Error(`execution target lease corpus unknown hostile kind ${vector.kind}`);
    if (executionTargetLeaseInstall(fixture, mutated, blake3Hex).outcome === "published") throw new Error(`execution target lease corpus published through ${vector.name}`);
    byteVectors += 1;
  }
  if (manifestFields < 30) throw new Error(`execution target lease corpus lost single-field substitutions: ${manifestFields}`);
  const statusCodes = Object.keys(fixture.expected.status).sort();
  if (JSON.stringify(statusCodes) !== JSON.stringify(leaseStatusKeys.slice().sort())) throw new Error("execution target lease corpus status vocabulary drifted");
  for (const [code, text] of Object.entries<{ en: string; de: string }>(fixture.expected.status)) {
    if (!text.en || !text.de || text.en === text.de) throw new Error(`execution target lease corpus status ${code} is not explicitly bilingual`);
    for (const fragment of fixture.expected.forbiddenStatusFragments) if (text.en.includes(fragment) || text.de.includes(fragment)) throw new Error(`execution target lease corpus status ${code} leaked ${fragment}`);
    const role = fixture.expected.statusRoles[code];
    if (role !== (code === "verifying" || code === "retrying" || code === "catching-up" ? "status" : "alert")) throw new Error(`execution target lease corpus status ${code} has the wrong live-region role`);
  }
  if (Object.values(fixture.expected.rendererClaims).some((claim) => claim !== false)) throw new Error("execution target lease corpus claims a renderer it does not have");
  console.log(
    `execution-target-lease-oracle: ajv=1 positive=1 manifest-fields=${manifestFields} byte-vectors=${byteVectors} lifecycle=${lifecycleVectors} hostile=${fixture.hostile.length} component-bytes=${component.length} descriptor-bytes=${descriptor.length} node+webcrypto-sha256=agree first-party-blake3=known-answer status=${statusCodes.length} passed`,
  );
}

function proveExecutionTargetLeaseSource(repoRoot: string): void {
  const worker = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"));
  const workerRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker");
  const region = ["🪪️execution-lease", "🧊️cold-pair", "📥️execution-target-read", "🧩️actor-wire-helpers", "🧷️actor-reservation", "🎭️actor-lifecycle"].map((child) => readFileSync(join(workerRoot, child, "🟦️.ts"), "utf8")).join("\n");
  if (region.length === 0) throw new Error("browser execution-target lease region is absent");
  for (const forbidden of ["loadPluginModule", "ActivationRegistry", "load_wasm_plugins", "attach_backbone"]) {
    if (region.includes(forbidden)) throw new Error(`browser execution-target lease region reached ${forbidden}`);
  }
  if (!region.includes('crypto.subtle.digest("SHA-256"') || !region.includes("blake3Hex(")) throw new Error("browser execution-target lease lost its Web Crypto SHA-256 or first-party BLAKE3 verification");
  if (!region.includes("descriptor.executionProtocol") || !region.includes("DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1")) throw new Error("browser execution-target lease lost compiled app-channel admission");
  if (!region.includes("state.docAbort.signal")) throw new Error("browser execution-target lease no longer shares the document cancellation scope");
  if (!worker.includes('plan.surface.rendererTarget !== "react" && (leaseFields === undefined')) throw new Error("browser plan authority no longer gates a non-react renderer on a live lease");
  const bin = readHubBinSource(repoRoot);
  for (const asset of ["manifest", "component", "descriptor", "browser-actor"]) {
    if (!bin.includes(`/spaces/{space_id}/documents/{id}/execution-target/${asset}`)) throw new Error(`hub execution-target ${asset} route is not mounted`);
  }
  if (!bin.includes("assets_for_current_selection(&descriptor, intent.requested_surface_id.as_deref(), writable, &generation_id)")) throw new Error("hub execution-target routes no longer resolve through the exact-selection accessor");
  const catalog = readTrustedCatalogSource(repoRoot);
  if (!catalog.includes("pub fn assets_for_current_selection(") || !catalog.includes("if current_generation != self.generation_id")) throw new Error("trusted catalog exact-selection accessor is absent or not generation-bound");
  const client = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), { crates: false });
  if (client.includes("matches_surface") || client.includes("DocumentSocketSurfaceExpectationV1")) throw new Error("native directory client kept a partial surface predicate");
  const sync = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"));
  if (!sync.includes("authority.matches_lease_fields(lease)")) throw new Error("native reconnect no longer compares the complete lease fields");
  console.log("execution-target-lease-source: browser lease region=renderer-free hub routes=4 accessor=generation-bound native=full-field passed");
}

export class ExecutionTargetLeaseCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some((segment) => segment !== "--native")) throw new Error("unsupported execution-target-lease argument");
    await proveExecutionTargetLeaseCorpus(this.repoRoot);
    proveExecutionTargetLeaseSource(this.repoRoot);
    if (!segments.includes("--native")) return;
    const env = { ...process.env, RUST_MIN_STACK: "268435456" };
    const exactLaw = (target: string[], suffix: string): string => {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", ...target, suffix, "--", "--list"], { cwd: this.root, env, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) {
        const diagnostic = listed.stderr.trim().slice(-4_000);
        throw new Error(`execution-target-lease-check expected exactly one ${suffix} law, selected ${matches.length}; status=${listed.status}; diagnostic=${diagnostic || "<none>"}`);
      }
      return matches[0]!;
    };
    const laws = [
      { target: ["-p", "semio-hub-trusted-catalog-laws", "--lib"], suffix: "selected_execution_target_assets_are_generation_and_digest_bound" },
      { target: ["-p", "semio-hub-open-plan-laws", "--lib"], suffix: "execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body" },
      { target: ["-p", "semio-framework-os-kernel", "--lib"], suffix: "execution_target_lease_compares_every_plan_and_verified_byte_field" },
    ].map((law) => ({ ...law, name: exactLaw(law.target, law.suffix) }));
    console.log(`execution-target-lease-laws: qualification=default-feature-subset exact=${laws.length} laws=${laws.map((law) => law.name).join(",")}`);
    for (const law of laws) runCargo(["test", "--manifest-path", "Cargo.toml", ...law.target, law.name, "--", "--exact", "--test-threads=1"], this.root, env);
    console.log("execution-target-lease-check: neutral corpus, source boundary and current native laws passed");
  }
}

export class ExecutionTargetLeaseBrowserCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveExecutionTargetLeaseCorpus(this.repoRoot);
    proveExecutionTargetLeaseSource(this.repoRoot);
    runCmd("bun", ["repo", "run", "@semio-tech/framework-os:test", "--skip-cache", "--", "long", "--run", "-t", "browser execution target lease|browser GIS viewer exposes localized renderer-unavailable"], {
      cwd: this.repoRoot,
      ...orchestratorBudgetOpts(),
    });
    console.log("execution-target-lease-browser-check: neutral corpus, source boundary and browser Worker verify/reject/renderer-unavailable runtime passed");
  }
}
