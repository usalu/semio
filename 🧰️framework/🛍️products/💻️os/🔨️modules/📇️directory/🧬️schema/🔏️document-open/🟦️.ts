/** 🔏️ Document open plan and execution-target lease admission: the strict parsers and full-field relations that hold a verified package projection to its shipped component receipt. The wire shapes are the types of the neutral directory schema; only these parsers depend on the receipt schema authority. */
import { type ArtifactFrontier, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES, DOCUMENT_OPEN_ID_MAX_BYTES, DOCUMENT_OPEN_PLAN_MAX_TTL_MS, type DocumentExecutionTargetLeaseFieldsV1, type DocumentOpenIntentV1, type DocumentOpenPackageV1, type DocumentOpenPlanV1, type DocumentPlanSocketGrantIntentV1, type DocumentScope, artifactFrontierIsEditedForV1, artifactFrontierIsGenesisForV1 } from "@semio-tech/framework/directory/schema";
import { parseDocumentOpenBrowserActorV1, parseDocumentExecutionTargetBrowserActorV1, documentBrowserActorLeaseFromPlanV1, sameDocumentBrowserActorV1 } from "@semio-tech/framework/directory/schema/browser-actor";
import { SHIPPED_COMPONENT_MAX_BYTES, componentReceiptBindsV1, componentReceiptValueV1, encodeComponentReceiptV1 } from "@semio-tech/framework-plugin/describe/component-receipt/schema";


/** 📦️ Strictly parses one package projection and holds its receipt to the package law: a valid shipped receipt naming exactly
 * this package, its component digest and its semantic descriptor digest. `error` prefixes every refusal. */
export function parseDocumentOpenPackageV1(value: unknown, error: string): DocumentOpenPackageV1 {
  const packageValue = documentOpenObject(value, ["pluginId", "packageId", "version", "componentSha256", "componentBlake3", "descriptorByteSha256", "executionProtocol", "componentReceipt"]);
  const executionProtocol = documentOpenObject(packageValue.executionProtocol, ["appChannelVersion"]);
  if (executionProtocol.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1) throw new Error(`${error}.unsupported-execution-protocol`);
  const parsed: DocumentOpenPackageV1 = {
    pluginId: documentOpenText(packageValue.pluginId),
    packageId: documentOpenText(packageValue.packageId),
    version: documentOpenText(packageValue.version),
    componentSha256: documentOpenHash(packageValue.componentSha256),
    componentBlake3: documentOpenHash(packageValue.componentBlake3),
    descriptorByteSha256: documentOpenHash(packageValue.descriptorByteSha256),
    executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 },
    componentReceipt: componentReceiptValueV1(packageValue.componentReceipt),
  };
  if (!componentReceiptBindsV1(parsed.componentReceipt, { profile: "release", pluginId: parsed.pluginId, packageId: parsed.packageId, wasmSha256: parsed.componentSha256, semanticDescriptorSha256: parsed.descriptorByteSha256 })) throw new Error(`${error}.unbound-receipt`);
  return parsed;
}

function documentOpenObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("document-open.invalid-object");
  const object = value as Record<string, unknown>;
  const accepted = new Set([...required, ...optional]);
  // 🩺️ The refusal names the keys it refused on. This check is the closed-object gate for the plan
  // and for every object nested inside it, and a bare code left a caller unable to tell an added
  // field from a missing one, in which object — including the common case where the body is not a
  // plan at all but a two-field `document-open-plan-error/v1`, whose `code` is the real answer.
  const unexpected = Object.keys(object).filter((key) => !accepted.has(key));
  const missing = required.filter((key) => !(key in object));
  if (unexpected.length !== 0 || missing.length !== 0) throw new Error(`document-open.invalid-fields unexpected=[${unexpected.join(",")}] missing=[${missing.join(",")}] accepted=[${[...accepted].join(",")}]`);
  return object;
}

function documentOpenText(value: unknown, maxBytes = DOCUMENT_OPEN_ID_MAX_BYTES): string {
  if (typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).length > maxBytes || /\p{Cc}/u.test(value)) throw new Error("document-open.invalid-text");
  return value;
}

function documentOpenHash(value: unknown): string {
  if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value) || /^0{64}$/u.test(value)) throw new Error("document-open.invalid-hash");
  return value;
}

function documentOpenInteger(value: unknown, positive = false): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < (positive ? 1 : 0)) throw new Error("document-open.invalid-integer");
  return value;
}

function documentOpenReceipt(value: unknown): string {
  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
  if (typeof value !== "string" || !/^open\.v1\.[A-Za-z0-9_-]{43}$/u.test(value) || (alphabet.indexOf(value.at(-1)!) & 0b11) !== 0) throw new Error("document-open.invalid-receipt");
  return value;
}

function parseDocumentOpenScope(value: unknown): DocumentScope {
  const object = documentOpenObject(value, ["spaceId", "documentId"]);
  return { spaceId: documentOpenText(object.spaceId), documentId: documentOpenText(object.documentId) };
}

function parseDocumentOpenFrontier(value: unknown, scope: DocumentScope): ArtifactFrontier {
  const object = documentOpenObject(value, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainHash"]);
  const chainHash = object.chainHash;
  if (!Array.isArray(chainHash) || chainHash.length !== 32 || chainHash.some((byte) => typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255))
    throw new Error("document-open.invalid-frontier-hash");
  const frontier = {
    documentId: documentOpenText(object.documentId),
    headEditOrdinal: documentOpenInteger(object.headEditOrdinal),
    headEditId: object.headEditId === "" ? "" : documentOpenText(object.headEditId),
    lastCommitSeq: documentOpenInteger(object.lastCommitSeq),
    chainHash: chainHash as number[],
  };
  if (!(artifactFrontierIsGenesisForV1(scope, frontier) || artifactFrontierIsEditedForV1(scope, frontier)) || frontier.lastCommitSeq > frontier.headEditOrdinal) throw new Error("document-open.stale-frontier");
  return frontier;
}

export function parseDocumentOpenIntentV1(value: unknown): DocumentOpenIntentV1 {
  const object = documentOpenObject(value, ["schema", "version", "scope", "clientInstanceId"], ["requestedSurfaceId"]);
  if (object.schema !== "semio.hub.document-open-intent/v1" || object.version !== 1) throw new Error("document-open.invalid-version");
  return {
    schema: object.schema,
    version: object.version,
    scope: parseDocumentOpenScope(object.scope),
    ...(object.requestedSurfaceId === undefined ? {} : { requestedSurfaceId: documentOpenText(object.requestedSurfaceId) }),
    clientInstanceId: documentOpenText(object.clientInstanceId, DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES),
  };
}

export function parseDocumentPlanSocketGrantIntentV1(value: unknown): DocumentPlanSocketGrantIntentV1 {
  const object = documentOpenObject(value, ["schema", "version", "planReceipt"]);
  if (object.schema !== "semio.hub.document-plan-socket-grant-intent/v1" || object.version !== 1) throw new Error("document-open.invalid-version");
  return { schema: object.schema, version: object.version, planReceipt: documentOpenReceipt(object.planReceipt) };
}

export function parseDocumentOpenPlanV1(value: unknown, nowMs: number): DocumentOpenPlanV1 {
  const object = documentOpenObject(value, ["schema", "version", "receipt", "expiresAtUnixMs", "scope", "descriptorDigestV1", "catalog", "package", "artifact", "parentDialect", "surface", "browserActor", "grant", "checkpoint", "revalidation"]);
  if (object.schema !== "semio.hub.document-open-plan/v1" || object.version !== 1) throw new Error("document-open.invalid-version");
  const scope = parseDocumentOpenScope(object.scope);
  const descriptorDigestV1 = documentOpenHash(object.descriptorDigestV1);
  const catalog = documentOpenObject(object.catalog, ["generationId"]);
  const parsedPackage = parseDocumentOpenPackageV1(object.package, "document-open");
  const artifact = documentOpenObject(object.artifact, ["kind", "schema", "packSchemaHash"]);
  const parentDialect = documentOpenObject(object.parentDialect, ["artifactKind", "standard", "subset"]);
  const surface = documentOpenObject(object.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
  const grant = documentOpenObject(object.grant, ["read", "write", "observe"]);
  const revalidation = documentOpenObject(object.revalidation, ["directoryRevision", "membershipGeneration"], ["sessionGeneration", "shareGeneration"]);
  const expiresAtUnixMs = documentOpenInteger(object.expiresAtUnixMs, true);
  if (expiresAtUnixMs <= nowMs || expiresAtUnixMs - nowMs > DOCUMENT_OPEN_PLAN_MAX_TTL_MS || (revalidation.sessionGeneration === undefined) === (revalidation.shareGeneration === undefined))
    throw new Error("document-open.expired-or-ambiguous-binding");
  if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean") throw new Error("document-open.invalid-grant");
  if ((surface.role !== "viewer" && surface.role !== "editor") || (surface.rendererTarget !== "react" && surface.rendererTarget !== "wgpu" && surface.rendererTarget !== "wasm") || grant.write !== (surface.role === "editor"))
    throw new Error("document-open.invalid-surface");
  const parsedParentDialect = {
    artifactKind: documentOpenText(parentDialect.artifactKind),
    standard: documentOpenText(parentDialect.standard),
    subset: documentOpenText(parentDialect.subset),
  };
  if (Object.values(parsedParentDialect).some((value) => value.trim() !== value)) throw new Error("document-open.invalid-parent-dialect");
  const checkpoint = documentOpenObject(object.checkpoint, ["checkpointId", "descriptorDigestV1", "baselineFrontier", "aggregateSha256"]);
  if (checkpoint.descriptorDigestV1 !== descriptorDigestV1) throw new Error("document-open.stale-checkpoint");
  return {
    schema: object.schema,
    version: object.version,
    receipt: documentOpenReceipt(object.receipt),
    expiresAtUnixMs,
    scope,
    descriptorDigestV1,
    catalog: { generationId: documentOpenHash(catalog.generationId) },
    package: parsedPackage,
    artifact: { kind: documentOpenText(artifact.kind), schema: documentOpenText(artifact.schema), packSchemaHash: documentOpenHash(artifact.packSchemaHash) },
    parentDialect: parsedParentDialect,
    surface: {
      surfaceId: documentOpenText(surface.surfaceId),
      appId: documentOpenText(surface.appId),
      windowKindId: documentOpenText(surface.windowKindId),
      role: surface.role,
      rendererTarget: surface.rendererTarget,
    },
    browserActor: parseDocumentOpenBrowserActorV1(object.browserActor, parsedPackage, surface.rendererTarget),
    grant: { read: true, write: grant.write, observe: true },
    checkpoint: {
      checkpointId: documentOpenHash(checkpoint.checkpointId),
      descriptorDigestV1,
      baselineFrontier: parseDocumentOpenFrontier(checkpoint.baselineFrontier, scope),
      aggregateSha256: documentOpenHash(checkpoint.aggregateSha256),
    },
    revalidation: {
      directoryRevision: documentOpenInteger(revalidation.directoryRevision, true),
      membershipGeneration: documentOpenInteger(revalidation.membershipGeneration, true),
      ...(revalidation.sessionGeneration === undefined ? {} : { sessionGeneration: documentOpenInteger(revalidation.sessionGeneration, true) }),
      ...(revalidation.shareGeneration === undefined ? {} : { shareGeneration: documentOpenInteger(revalidation.shareGeneration, true) }),
    },
  };
}

//#region 🪪️ExecutionTargetLease
/** 🧯️ Exact maximum accepted bytes for one verified execution-target component — the shipped `release` cap every shipped
 * component's receipt is held to, the trusted catalog's own `TRUSTED_COMPONENT_MAX_BYTES`, shared verbatim by every transport. */
export const DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES = SHIPPED_COMPONENT_MAX_BYTES;

function documentExecutionTargetByteLength(value: unknown, maxBytes: number): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 1 || value > maxBytes) throw new Error("document-execution-target-lease.invalid-byte-length");
  return value;
}

/** ✅️ Strictly parses the public lease fields and enforces every byte/identity invariant: both
 * component digests and the descriptor digest must equal the package projection, the parent dialect
 * must share the artifact kind, the grant must follow the surface role, and the required checkpoint
 * must carry the same descriptor digest. */
export function parseDocumentExecutionTargetLeaseFieldsV1(value: unknown): DocumentExecutionTargetLeaseFieldsV1 {
  const object = documentOpenObject(value, ["schema", "version", "scope", "descriptorDigestV1", "catalog", "package", "component", "descriptor", "browserActor", "artifact", "parentDialect", "surface", "grant", "checkpoint", "revalidation"]);
  if (object.schema !== "semio.os.document-execution-target-lease/v1" || object.version !== 1) throw new Error("document-execution-target-lease.invalid-version");
  const scope = parseDocumentOpenScope(object.scope);
  const descriptorDigestV1 = documentOpenHash(object.descriptorDigestV1);
  const catalog = documentOpenObject(object.catalog, ["generationId"]);
  const parsedPackage = parseDocumentOpenPackageV1(object.package, "document-execution-target-lease");
  const component = documentOpenObject(object.component, ["sha256", "blake3", "byteLength"]);
  const descriptor = documentOpenObject(object.descriptor, ["sha256", "byteLength"]);
  const artifact = documentOpenObject(object.artifact, ["kind", "schema", "packSchemaHash"]);
  const parentDialect = documentOpenObject(object.parentDialect, ["artifactKind", "standard", "subset"]);
  const surface = documentOpenObject(object.surface, ["surfaceId", "appId", "windowKindId", "role", "rendererTarget"]);
  const grant = documentOpenObject(object.grant, ["read", "write", "observe"]);
  const revalidation = documentOpenObject(object.revalidation, ["directoryRevision", "membershipGeneration"], ["sessionGeneration", "shareGeneration"]);
  if ((revalidation.sessionGeneration === undefined) === (revalidation.shareGeneration === undefined)) throw new Error("document-execution-target-lease.ambiguous-binding");
  if (grant.read !== true || grant.observe !== true || typeof grant.write !== "boolean") throw new Error("document-execution-target-lease.invalid-grant");
  if ((surface.role !== "viewer" && surface.role !== "editor") || (surface.rendererTarget !== "react" && surface.rendererTarget !== "wgpu" && surface.rendererTarget !== "wasm") || grant.write !== (surface.role === "editor"))
    throw new Error("document-execution-target-lease.invalid-surface");
  const parsedParentDialect = {
    artifactKind: documentOpenText(parentDialect.artifactKind),
    standard: documentOpenText(parentDialect.standard),
    subset: documentOpenText(parentDialect.subset),
  };
  if (Object.values(parsedParentDialect).some((entry) => entry.trim() !== entry)) throw new Error("document-execution-target-lease.invalid-parent-dialect");
  const parsedComponent = {
    sha256: documentOpenHash(component.sha256),
    blake3: documentOpenHash(component.blake3),
    byteLength: documentExecutionTargetByteLength(component.byteLength, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES),
  };
  const parsedDescriptor = {
    sha256: documentOpenHash(descriptor.sha256),
    byteLength: documentExecutionTargetByteLength(descriptor.byteLength, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES),
  };
  if (parsedComponent.sha256 !== parsedPackage.componentSha256 || parsedComponent.blake3 !== parsedPackage.componentBlake3 || parsedDescriptor.sha256 !== parsedPackage.descriptorByteSha256 || parsedComponent.byteLength !== parsedPackage.componentReceipt.wasmBytes)
    throw new Error("document-execution-target-lease.unbound-bytes");
  const checkpoint = documentOpenObject(object.checkpoint, ["checkpointId", "descriptorDigestV1", "baselineFrontier", "aggregateSha256"]);
  if (checkpoint.descriptorDigestV1 !== descriptorDigestV1) throw new Error("document-execution-target-lease.stale-checkpoint");
  return {
    schema: object.schema,
    version: object.version,
    scope,
    descriptorDigestV1,
    catalog: { generationId: documentOpenHash(catalog.generationId) },
    package: parsedPackage,
    component: parsedComponent,
    descriptor: parsedDescriptor,
    browserActor: parseDocumentExecutionTargetBrowserActorV1(object.browserActor, parsedPackage, surface.rendererTarget),
    artifact: { kind: documentOpenText(artifact.kind), schema: documentOpenText(artifact.schema), packSchemaHash: documentOpenHash(artifact.packSchemaHash) },
    parentDialect: parsedParentDialect,
    surface: {
      surfaceId: documentOpenText(surface.surfaceId),
      appId: documentOpenText(surface.appId),
      windowKindId: documentOpenText(surface.windowKindId),
      role: surface.role,
      rendererTarget: surface.rendererTarget,
    },
    grant: { read: true, write: grant.write, observe: true },
    checkpoint: {
      checkpointId: documentOpenHash(checkpoint.checkpointId),
      descriptorDigestV1,
      baselineFrontier: parseDocumentOpenFrontier(checkpoint.baselineFrontier, scope),
      aggregateSha256: documentOpenHash(checkpoint.aggregateSha256),
    },
    revalidation: {
      directoryRevision: documentOpenInteger(revalidation.directoryRevision, true),
      membershipGeneration: documentOpenInteger(revalidation.membershipGeneration, true),
      ...(revalidation.sessionGeneration === undefined ? {} : { sessionGeneration: documentOpenInteger(revalidation.sessionGeneration, true) }),
      ...(revalidation.shareGeneration === undefined ? {} : { shareGeneration: documentOpenInteger(revalidation.shareGeneration, true) }),
    },
  };
}

/** 🧾️ Projects one already-validated plan into receipt-free lease fields. The plan constrains every
 * identity but no byte length, so both lengths come from the installation being compared and are
 * independently enforced against the exact streamed bytes before a lease is ever minted. */
export function leaseFieldsFromPlanV1(plan: DocumentOpenPlanV1, byteLengths: { readonly component: number; readonly descriptor: number; readonly browserActor?: number }): DocumentExecutionTargetLeaseFieldsV1 {
  return parseDocumentExecutionTargetLeaseFieldsV1({
    schema: "semio.os.document-execution-target-lease/v1",
    version: 1,
    scope: plan.scope,
    descriptorDigestV1: plan.descriptorDigestV1,
    catalog: plan.catalog,
    package: plan.package,
    component: { sha256: plan.package.componentSha256, blake3: plan.package.componentBlake3, byteLength: byteLengths.component },
    descriptor: { sha256: plan.package.descriptorByteSha256, byteLength: byteLengths.descriptor },
    browserActor: documentBrowserActorLeaseFromPlanV1(plan.browserActor, plan.package, plan.surface.rendererTarget, byteLengths.browserActor),
    artifact: plan.artifact,
    parentDialect: plan.parentDialect,
    surface: plan.surface,
    grant: plan.grant,
    checkpoint: plan.checkpoint,
    revalidation: plan.revalidation,
  });
}

/** ⚖️ The one shared full-field lease relation. Every transport compares every field through it; a
 * browser or native subset comparison is never permitted. */
export function sameLeaseFieldsV1(left: DocumentExecutionTargetLeaseFieldsV1, right: DocumentExecutionTargetLeaseFieldsV1): boolean {
  const sameCheckpoint =
    left.checkpoint === undefined || right.checkpoint === undefined
      ? left.checkpoint === right.checkpoint
      : left.checkpoint.checkpointId === right.checkpoint.checkpointId &&
        left.checkpoint.descriptorDigestV1 === right.checkpoint.descriptorDigestV1 &&
        left.checkpoint.aggregateSha256 === right.checkpoint.aggregateSha256 &&
        left.checkpoint.baselineFrontier.documentId === right.checkpoint.baselineFrontier.documentId &&
        left.checkpoint.baselineFrontier.headEditOrdinal === right.checkpoint.baselineFrontier.headEditOrdinal &&
        left.checkpoint.baselineFrontier.headEditId === right.checkpoint.baselineFrontier.headEditId &&
        left.checkpoint.baselineFrontier.lastCommitSeq === right.checkpoint.baselineFrontier.lastCommitSeq &&
        left.checkpoint.baselineFrontier.chainHash.length === right.checkpoint.baselineFrontier.chainHash.length &&
        left.checkpoint.baselineFrontier.chainHash.every((byte, index) => byte === right.checkpoint!.baselineFrontier.chainHash[index]);
  return (
    left.schema === right.schema &&
    left.version === right.version &&
    left.scope.spaceId === right.scope.spaceId &&
    left.scope.documentId === right.scope.documentId &&
    left.descriptorDigestV1 === right.descriptorDigestV1 &&
    left.catalog.generationId === right.catalog.generationId &&
    left.package.pluginId === right.package.pluginId &&
    left.package.packageId === right.package.packageId &&
    left.package.version === right.package.version &&
    left.package.componentSha256 === right.package.componentSha256 &&
    left.package.componentBlake3 === right.package.componentBlake3 &&
    left.package.descriptorByteSha256 === right.package.descriptorByteSha256 &&
    left.package.executionProtocol.appChannelVersion === right.package.executionProtocol.appChannelVersion &&
    encodeComponentReceiptV1(left.package.componentReceipt) === encodeComponentReceiptV1(right.package.componentReceipt) &&
    left.component.sha256 === right.component.sha256 &&
    left.component.blake3 === right.component.blake3 &&
    left.component.byteLength === right.component.byteLength &&
    left.descriptor.sha256 === right.descriptor.sha256 &&
    left.descriptor.byteLength === right.descriptor.byteLength &&
    sameDocumentBrowserActorV1(left.browserActor, right.browserActor) &&
    left.artifact.kind === right.artifact.kind &&
    left.artifact.schema === right.artifact.schema &&
    left.artifact.packSchemaHash === right.artifact.packSchemaHash &&
    left.parentDialect.artifactKind === right.parentDialect.artifactKind &&
    left.parentDialect.standard === right.parentDialect.standard &&
    left.parentDialect.subset === right.parentDialect.subset &&
    left.surface.surfaceId === right.surface.surfaceId &&
    left.surface.appId === right.surface.appId &&
    left.surface.windowKindId === right.surface.windowKindId &&
    left.surface.role === right.surface.role &&
    left.surface.rendererTarget === right.surface.rendererTarget &&
    left.grant.read === right.grant.read &&
    left.grant.write === right.grant.write &&
    left.grant.observe === right.grant.observe &&
    sameCheckpoint &&
    left.revalidation.directoryRevision === right.revalidation.directoryRevision &&
    left.revalidation.membershipGeneration === right.revalidation.membershipGeneration &&
    left.revalidation.sessionGeneration === right.revalidation.sessionGeneration &&
    left.revalidation.shareGeneration === right.revalidation.shareGeneration
  );
}

/** ⏯️ Whether `next` — the fields a reconnect's fresh plan projects — names the SAME execution target under the SAME grant as `current`,
 * the lease of a child a link shortage suspended, so that child resumes instead of reopening. Two fields legitimately advance while a
 * document stays open and are adopted, only ever forward: the active `checkpoint` (the hub cut a newer one; the child is already
 * past it) and the `revalidation` witness (its `directoryRevision` is the hub directory's global head, which any directory event on
 * the hub moves). Every other field goes through {@link sameLeaseFieldsV1} — the full-field relation plus an explicit monotone
 * freshness rule, never a subset comparison. Before, a resume demanded the old witness verbatim, so on a shared hub every short
 * link cut reopened the document (ticket 26/09/23 C12, run `c12short5`).
 * @see ../../../🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json */
export function sameExecutionTargetV1(current: DocumentExecutionTargetLeaseFieldsV1, next: DocumentExecutionTargetLeaseFieldsV1): boolean {
  const forward = (from: number | undefined, to: number | undefined): boolean => (from === undefined ? to === undefined : to !== undefined && to >= from);
  const checkpointForward =
    current.checkpoint === undefined || next.checkpoint === undefined
      ? current.checkpoint === next.checkpoint
      : next.checkpoint.descriptorDigestV1 === current.checkpoint.descriptorDigestV1 &&
        next.checkpoint.baselineFrontier.documentId === current.checkpoint.baselineFrontier.documentId &&
        forward(current.checkpoint.baselineFrontier.headEditOrdinal, next.checkpoint.baselineFrontier.headEditOrdinal) &&
        forward(current.checkpoint.baselineFrontier.lastCommitSeq, next.checkpoint.baselineFrontier.lastCommitSeq);
  return (
    checkpointForward &&
    forward(current.revalidation.directoryRevision, next.revalidation.directoryRevision) &&
    forward(current.revalidation.membershipGeneration, next.revalidation.membershipGeneration) &&
    forward(current.revalidation.sessionGeneration, next.revalidation.sessionGeneration) &&
    forward(current.revalidation.shareGeneration, next.revalidation.shareGeneration) &&
    sameLeaseFieldsV1(current, { ...next, checkpoint: current.checkpoint, revalidation: current.revalidation })
  );
}
//#endregion 🪪️ExecutionTargetLease
