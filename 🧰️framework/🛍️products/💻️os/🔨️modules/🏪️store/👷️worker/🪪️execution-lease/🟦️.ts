import type { DocumentExecutionTargetLeaseFieldsV1, DocumentExecutionTargetProgressV1, DocumentOpenIntentV1, DocumentOpenPlanV1 } from "@semio-tech/framework/directory/schema";
import type { DocumentSocketGrantReceiptV1, PersistenceBinding } from "../../../../🟦️";
import { parseDocumentSocketGrantReceiptV1, decodePackValue, packValueToExactJson, encodePackValue } from "../../../../🟦️";
import { sameExecutionTargetV1, parseDocumentExecutionTargetLeaseFieldsV1, sameLeaseFieldsV1, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "../../../📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
import { assertBrowserActorDescribeCapacityV1, verifyBrowserActorDescribeV1 } from "../../../🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts";
import { type AdmittedDocumentServiceDeclarationV1, DOCUMENT_SERVICE_TOPIC_V1 } from "@semio-tech/framework/inference/service";
import type { PackValue } from "../../../../🟦️";
import { admitDocumentServiceWireDeclarationV1 } from "@semio-tech/framework/inference/service/io";
import type { BrowserActorChildValue } from "../../../🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts";
import { requestExecutionTargetAssetV1, executionTargetRetrySleepV1 } from "../🔁️execution-target-retry/🟦️.ts";
import { DOCUMENT_BROWSER_ACTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { measureChildValue, BROWSER_ACTOR_CHILD_LIMITS } from "@semio-tech/framework/browser-bundle/child/schema";
import type { FetchTimeoutResponse } from "@semio-tech/framework";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { blake3Hex } from "@semio-tech/framework";
import { EXECUTION_TARGET_MANIFEST_MAX_BYTES, SOCKET_GRANT_REQUEST_TIMEOUT_MS, documentExecutionTargetLeaseBrand, documentExecutionTargetLeaseMintToken } from "../⚙️constants/🟦️.ts";
import type { DocumentBrowserActorChild, DocumentBrowserActorGrant, DocumentBrowserActorOpen } from "../🧩️actor-wire-helpers/🟦️.ts";
import { type DocumentRenderSurfacesV1, type ExecutionTargetReadControl, assertExecutionTargetRead, emitExecutionTargetStatus, executionTargetAssetPath, executionTargetSha256Hex, parseVerifiedPackageDescriptorV1, readExecutionTargetBody, readExecutionTargetJson, receiptFreeFields } from "../📥️execution-target-read/🟦️.ts";
import type { DocumentExecutionTargetAssetV1 } from "../🧊️cold-pair/🟦️.ts";
import { hubSessionFetch } from "../🔐️hub-session/🟦️.ts";
import type { ArtifactState } from "../🧾️document-state/🟦️.ts";

/** 🪪️ Private non-serializable owner of one verified execution target. Its constructor is
 * unreachable outside this module (a module-private mint token), it owns the verified component and
 * descriptor buffers plus any private module URL, and public callers only ever receive a frozen copy
 * of {@link DocumentExecutionTargetLeaseFieldsV1}. It is never posted, cloned or encoded. */
export class DocumentExecutionTargetLease {
  readonly [documentExecutionTargetLeaseBrand] = true;
  #fields: DocumentExecutionTargetLeaseFieldsV1;
  #hubOrigin: string;
  #component: Uint8Array | null;
  #descriptor: Uint8Array | null;
  #moduleUrl: string | null = null;
  #live = true;
  #retirement = new AbortController();
  #browserActorGrant: DocumentBrowserActorGrant | null = null;
  #browserActorOpen: DocumentBrowserActorOpen | null = null;
  #renderSurfaces: DocumentRenderSurfacesV1 | null = null;

  constructor(token: symbol, fields: DocumentExecutionTargetLeaseFieldsV1, hubOrigin: string, component: Uint8Array, descriptor: Uint8Array) {
    if (token !== documentExecutionTargetLeaseMintToken) throw new Error("document execution target lease: private constructor");
    this.#fields = fields;
    this.#hubOrigin = hubOrigin;
    this.#component = component;
    this.#descriptor = descriptor;
    Object.freeze(this);
  }

  get live(): boolean {
    return this.#live;
  }

  get hubOrigin(): string {
    return this.#hubOrigin;
  }

  fields(): DocumentExecutionTargetLeaseFieldsV1 {
    if (!this.#live) throw new Error("document execution target lease: dropped");
    return Object.freeze(structuredClone(this.#fields));
  }

  /** 🪟️ The verified window and panel bodies this target's actor child renders (see {@link DocumentRenderSurfacesV1}). */
  renderSurfaces(): DocumentRenderSurfacesV1 {
    if (!this.#live || !this.#descriptor) throw new Error("document browser actor: dropped render descriptor");
    this.#renderSurfaces ??= parseVerifiedPackageDescriptorV1(this.#descriptor, this.#fields);
    return this.#renderSurfaces;
  }

  get retirement(): AbortSignal {
    return this.#retirement.signal;
  }

  admitBrowserActor(token: symbol, receipt: DocumentSocketGrantReceiptV1, retireAtMs: number, open: DocumentBrowserActorOpen): void {
    if (token !== documentExecutionTargetLeaseMintToken || !this.#live || this.#browserActorGrant !== null || !Number.isSafeInteger(retireAtMs)) throw new Error("document browser actor: private grant");
    const parsed = parseDocumentSocketGrantReceiptV1(receipt);
    if (parsed.expiresAtMs > retireAtMs) throw new Error("document browser actor: invalid grant");
    open.assertCurrent();
    this.#browserActorOpen = Object.freeze({ binding: structuredClone(open.binding), intent: structuredClone(open.intent), assertCurrent: open.assertCurrent });
    this.#browserActorGrant = Object.freeze({ actorId: parsed.actorId, reserveBeforeMs: parsed.expiresAtMs, retireAtMs });
  }

  /** ⏯️ Adopts the fields a reconnect's fresh plan projects for the SAME execution target ({@link sameExecutionTargetV1}): the
   * newer active checkpoint and revalidation witness a suspended child resumes under. Anything else is another target. */
  refresh(token: symbol, fields: DocumentExecutionTargetLeaseFieldsV1): void {
    if (token !== documentExecutionTargetLeaseMintToken || !this.#live || !sameExecutionTargetV1(this.#fields, fields)) throw new Error("document execution target lease: another target");
    this.#fields = structuredClone(fields);
  }

  /** 🔁️ Re-admits the SAME hub actor on a fresh socket grant after a short link loss, keeping the verified target and
   * the live child: the hub binds a session's actor to the session, so a grant naming another actor is refused. */
  readmitBrowserActor(token: symbol, receipt: DocumentSocketGrantReceiptV1, retireAtMs: number, open: DocumentBrowserActorOpen): DocumentBrowserActorGrant {
    const current = this.#browserActorGrant;
    if (token !== documentExecutionTargetLeaseMintToken || !this.#live || current === null || !Number.isSafeInteger(retireAtMs)) throw new Error("document browser actor: private grant");
    const parsed = parseDocumentSocketGrantReceiptV1(receipt);
    if (parsed.actorId !== current.actorId || parsed.expiresAtMs > retireAtMs) throw new Error("document browser actor: invalid grant");
    open.assertCurrent();
    this.#browserActorOpen = Object.freeze({ binding: structuredClone(open.binding), intent: structuredClone(open.intent), assertCurrent: open.assertCurrent });
    this.#browserActorGrant = Object.freeze({ actorId: parsed.actorId, reserveBeforeMs: parsed.expiresAtMs, retireAtMs });
    return this.#browserActorGrant;
  }

  assertBrowserActorCurrent(): void {
    if (!this.#live || !this.#browserActorOpen) throw new Error("document browser actor: retired open owner");
    this.#browserActorOpen.assertCurrent();
  }

  assertBrowserActorDescribeCapacity(): void {
    if (!this.#live || !this.#descriptor) throw new Error("document browser actor: dropped descriptor");
    assertBrowserActorDescribeCapacityV1(this.#descriptor.byteLength);
  }

  /** 📜 Reads installed service declarations only from this verified immutable owner descriptor. */
  documentServices(): readonly AdmittedDocumentServiceDeclarationV1[] {
    if (!this.#live || this.#descriptor === null) throw new Error("installed-service.owner-retired");
    const descriptor = decodePackValue(this.#descriptor) as Readonly<Record<string, PackValue>>;
    const manifest = descriptor.manifest as Readonly<Record<string, PackValue>>;
    const topics = manifest.topicContributions;
    if (topics === undefined) return [];
    if (!Array.isArray(topics) || topics.length > 64) throw new Error("installed-service.invalid-declarations");
    return topics.flatMap((value) => {
      if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("installed-service.invalid-declarations");
      const topic = value as Readonly<Record<string, PackValue>>;
      return topic.topic === DOCUMENT_SERVICE_TOPIC_V1 ? [admitDocumentServiceWireDeclarationV1(this.#fields.package.pluginId, packValueToExactJson(topic.payload!))] : [];
    });
  }

  async activateBrowserActor(child: DocumentBrowserActorChild, signal: AbortSignal, assertSession: () => void, report: (progress: DocumentExecutionTargetProgressV1) => void, retry: (progress: DocumentExecutionTargetProgressV1) => void): Promise<void> {
    const open = this.#browserActorOpen,
      grant = this.#browserActorGrant,
      actor = this.#fields.browserActor;
    if (!open || !grant || actor.kind !== "closed-browser-actor") throw new Error("document browser actor: missing private authority");
    const assertCurrent = () => {
      assertSession();
      open.assertCurrent();
      if (!this.#live || this.#browserActorOpen !== open || this.#browserActorGrant !== grant) throw new Error("document browser actor: retired authority");
    };
    const readControl = (): ExecutionTargetReadControl => ({ signal, deadlineAtMs: Math.min(grant.reserveBeforeMs, grant.retireAtMs, Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS), assertCurrent });
    let control = readControl();
    let source: Uint8Array | undefined, result: BrowserActorChildValue | undefined;
    try {
      assertExecutionTargetRead(control);
      this.assertBrowserActorDescribeCapacity();
      const response = await requestExecutionTargetAssetV1(() => browserExecutionTargetAssetRequest(open.binding, open.intent.scope.documentId, "browser-actor", open.intent, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal }), {
        signal,
        sleep: executionTargetRetrySleepV1,
        onRetry: (attempt, of) => {
          assertCurrent();
          retry({ stage: "browser-actor", completedBytes: attempt, totalBytes: of });
        },
      });
      control = readControl();
      source = await readExecutionTargetBody(response, actor.byteLength, DOCUMENT_BROWSER_ACTOR_MAX_BYTES, "browser-actor", control, report);
      assertExecutionTargetRead(control);
      if ((await executionTargetSha256Hex(source)) !== actor.sha256) throw new Error("document browser actor: body integrity");
      assertExecutionTargetRead(control);
      report({ stage: "actor-transfer", completedBytes: 0, totalBytes: actor.byteLength });
      await child.load(source.buffer as ArrayBuffer, (progress) => report({ stage: `actor-${progress.stage}`, completedBytes: progress.completedBytes, totalBytes: progress.totalBytes }));
      assertExecutionTargetRead(control);
      report({ stage: "actor-describe", completedBytes: 0, totalBytes: 1 });
      result = await child.invoke(["describe", "describe"], []);
      report({ stage: "actor-describe", completedBytes: 1, totalBytes: 1 });
      assertExecutionTargetRead(control);
      if (!(result instanceof Uint8Array) || !(result.buffer instanceof ArrayBuffer) || result.byteOffset !== 0 || result.byteLength !== result.buffer.byteLength || !this.#descriptor) throw new Error("document browser actor: describe shape");
      report({ stage: "actor-verify", completedBytes: 0, totalBytes: 1 });
      verifyBrowserActorDescribeV1(result, this.#descriptor, { decode: decodePackValue, encode: encodePackValue });
      report({ stage: "actor-verify", completedBytes: 1, totalBytes: 1 });
      assertExecutionTargetRead(control);
    } finally {
      if (source?.byteLength) source.fill(0);
      if (result !== undefined) for (const buffer of measureChildValue(result, BROWSER_ACTOR_CHILD_LIMITS.outputBytes).transfers) new Uint8Array(buffer).fill(0);
    }
  }

  browserActorGrant(): DocumentBrowserActorGrant | null {
    return this.#live ? this.#browserActorGrant : null;
  }

  drop(): void {
    if (!this.#live) return;
    this.#live = false;
    this.#retirement.abort();
    this.#browserActorGrant = null;
    this.#browserActorOpen = null;
    this.#component?.fill(0);
    this.#descriptor?.fill(0);
    this.#component = null;
    this.#descriptor = null;
    this.#renderSurfaces = null;
    if (this.#moduleUrl !== null) {
      URL.revokeObjectURL(this.#moduleUrl);
      this.#moduleUrl = null;
    }
  }
}

/** 🚪️ The only broker operation this lane owns: exactly the four protected document-scoped asset
 * calls, always POST, always the bounded `DocumentOpenIntentV1` body, never a package, digest,
 * generation, path or receipt selector. Anything else is denied before a request exists. */
export function browserExecutionTargetAssetRequest(
  binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>,
  documentId: string,
  asset: DocumentExecutionTargetAssetV1,
  intent: DocumentOpenIntentV1,
  options: { readonly timeoutMs: number; readonly signal: AbortSignal },
): Promise<FetchTimeoutResponse> {
  const path = executionTargetAssetPath(binding.spaceId, documentId, asset);
  if (!/^\/spaces\/[^/?#]+\/documents\/[^/?#]+\/execution-target\/(?:manifest|component|descriptor|browser-actor)$/u.test(path)) return Promise.reject(new Error("document execution target: operation denied"));
  if (intent.scope.spaceId !== binding.spaceId || intent.scope.documentId !== documentId) return Promise.reject(new Error("document execution target: operation denied"));
  return hubSessionFetch(`${path}`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(intent) }, options);
}

/** 🛡️ Acquires the server-selected verified execution target for one live plan and mints the private
 * lease only after every byte verifies. It shares {@link ArtifactState.docAbort} and the current
 * document-open deadline with {@link requestDocumentSocketAuthority}; a mismatch, cancellation or
 * deadline wipes every retained buffer and returns no lease. */
export async function installDocumentExecutionTargetLease(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>, plan: DocumentOpenPlanV1, intent: DocumentOpenIntentV1, assertOwner: () => void): Promise<DocumentExecutionTargetLease> {
  const signal = state.docAbort.signal;
  const options = { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal } as const;
  const readControl = (): ExecutionTargetReadControl => ({ signal, deadlineAtMs: Math.min(plan.expiresAtUnixMs, Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS), assertCurrent: assertOwner });
  const report = (progress: DocumentExecutionTargetProgressV1): void => emitExecutionTargetStatus(state, binding, "verifying", progress);
  const request = (asset: "manifest" | "component" | "descriptor"): Promise<FetchTimeoutResponse> =>
    requestExecutionTargetAssetV1(() => browserExecutionTargetAssetRequest(binding, state.config.documentId, asset, intent, options), {
      signal,
      sleep: executionTargetRetrySleepV1,
      onRetry: (attempt, of) => {
        assertOwner();
        emitExecutionTargetStatus(state, binding, "retrying", { stage: asset, completedBytes: attempt, totalBytes: of });
      },
    });
  let component: Uint8Array | null = null;
  let descriptorBytes: Uint8Array | null = null;
  try {
    if (signal.aborted) throw new Error("document execution target: cancelled");
    report({ stage: "manifest", completedBytes: 0, totalBytes: 1 });
    assertExecutionTargetRead(readControl());
    const manifestResponse = await request("manifest");
    const manifestControl = readControl();
    const fields = parseDocumentExecutionTargetLeaseFieldsV1(await readExecutionTargetJson(manifestResponse, EXECUTION_TARGET_MANIFEST_MAX_BYTES, manifestControl));
    if (
      !sameLeaseFieldsV1(
        fields,
        receiptFreeFields(plan, { component: fields.component.byteLength, descriptor: fields.descriptor.byteLength, browserActor: fields.browserActor.kind === "closed-browser-actor" ? fields.browserActor.byteLength : undefined }),
      )
    )
      throw new Error("document execution target: manifest mismatch");
    report({ stage: "manifest", completedBytes: 1, totalBytes: 1 });

    if (signal.aborted) throw new Error("document execution target: cancelled");
    assertExecutionTargetRead(readControl());
    const componentResponse = await request("component");
    const componentControl = readControl();
    component = await readExecutionTargetBody(componentResponse, fields.component.byteLength, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, "component", componentControl, report);

    if (signal.aborted) throw new Error("document execution target: cancelled");
    assertExecutionTargetRead(readControl());
    const descriptorResponse = await request("descriptor");
    const descriptorControl = readControl();
    descriptorBytes = await readExecutionTargetBody(descriptorResponse, fields.descriptor.byteLength, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "descriptor", descriptorControl, report);

    if (signal.aborted) throw new Error("document execution target: cancelled");
    const verifyControl = readControl();
    assertExecutionTargetRead(verifyControl);
    report({ stage: "verify", completedBytes: 0, totalBytes: 3 });
    if ((await executionTargetSha256Hex(component)) !== fields.component.sha256) throw new Error("document execution target: component integrity");
    assertExecutionTargetRead(verifyControl);
    report({ stage: "verify", completedBytes: 1, totalBytes: 3 });
    if (blake3Hex(component) !== fields.component.blake3) throw new Error("document execution target: component integrity");
    assertExecutionTargetRead(verifyControl);
    report({ stage: "verify", completedBytes: 2, totalBytes: 3 });
    if ((await executionTargetSha256Hex(descriptorBytes)) !== fields.descriptor.sha256) throw new Error("document execution target: descriptor integrity");
    parseVerifiedPackageDescriptorV1(descriptorBytes, fields);
    assertExecutionTargetRead(verifyControl);
    report({ stage: "verify", completedBytes: 3, totalBytes: 3 });
    if (signal.aborted) throw new Error("document execution target: cancelled");
    assertExecutionTargetRead(verifyControl);
    const lease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, fields, binding.baseUrl.replace(/\/+$/u, ""), component, descriptorBytes);
    component = null;
    descriptorBytes = null;
    return lease;
  } finally {
    component?.fill(0);
    descriptorBytes?.fill(0);
  }
}
