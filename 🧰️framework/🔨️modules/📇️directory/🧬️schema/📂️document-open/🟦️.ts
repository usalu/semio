/** 📂️ Document open plan: intent, catalog, package, surface, grants, checkpoint, revalidation and the typed plan errors. */

import { type DocumentOpenBrowserActorV1 } from "../🌐️browser-actor/🟦️.ts";
import type { ComponentReceiptV1 } from "../../../🏷️component-receipt/🟦️.ts";
import { type ArtifactFrontier, type DocumentScope } from "../../../📄️document/🪪️scope/🟦️.ts";

//#region 🔖️Views
export const DOCUMENT_OPEN_CLIENT_INSTANCE_MAX_BYTES = 128;
export const DOCUMENT_OPEN_PLAN_MAX_TTL_MS = 30_000;

export interface DocumentOpenIntentV1 {
  schema: "semio.hub.document-open-intent/v1";
  version: 1;
  scope: DocumentScope;
  requestedSurfaceId?: string;
  clientInstanceId: string;
}

export type DocumentOpenRendererTargetV1 = "react" | "wgpu" | "wasm";
export type DocumentOpenSurfaceRoleV1 = "viewer" | "editor";

export interface DocumentOpenCatalogV1 {
  generationId: string;
}

export const DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 = 23;

export interface DocumentExecutionProtocolV1 {
  appChannelVersion: typeof DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1;
}

/** 📦️ Exact verified package projection: its identity, the byte identity of its component and semantic descriptor, and the
 * shipped `ComponentReceiptV1` binding the two. */
export interface DocumentOpenPackageV1 {
  pluginId: string;
  packageId: string;
  version: string;
  componentSha256: string;
  componentBlake3: string;
  descriptorByteSha256: string;
  executionProtocol: DocumentExecutionProtocolV1;
  componentReceipt: ComponentReceiptV1;
}

export interface DocumentOpenArtifactV1 {
  kind: string;
  schema: string;
  packSchemaHash: string;
}

export interface DocumentOpenParentDialectV1 {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface DocumentOpenSurfaceV1 {
  surfaceId: string;
  appId: string;
  windowKindId: string;
  role: DocumentOpenSurfaceRoleV1;
  rendererTarget: DocumentOpenRendererTargetV1;
}

export interface DocumentOpenGrantV1 {
  read: true;
  write: boolean;
  observe: true;
}

export interface DocumentOpenCheckpointV1 {
  checkpointId: string;
  descriptorDigestV1: string;
  baselineFrontier: ArtifactFrontier;
  aggregateSha256: string;
}

export interface DocumentOpenRevalidationV1 {
  directoryRevision: number;
  membershipGeneration: number;
  sessionGeneration?: number;
  shareGeneration?: number;
}

export interface DocumentOpenPlanV1 {
  schema: "semio.hub.document-open-plan/v1";
  version: 1;
  receipt: string;
  expiresAtUnixMs: number;
  scope: DocumentScope;
  descriptorDigestV1: string;
  catalog: DocumentOpenCatalogV1;
  package: DocumentOpenPackageV1;
  artifact: DocumentOpenArtifactV1;
  parentDialect: DocumentOpenParentDialectV1;
  surface: DocumentOpenSurfaceV1;
  browserActor: DocumentOpenBrowserActorV1;
  grant: DocumentOpenGrantV1;
  checkpoint: DocumentOpenCheckpointV1;
  revalidation: DocumentOpenRevalidationV1;
}

export interface DocumentPlanSocketGrantIntentV1 {
  schema: "semio.hub.document-plan-socket-grant-intent/v1";
  version: 1;
  planReceipt: string;
}

export type DocumentOpenPlanErrorCodeV1 = "denied" | "not-found" | "catalog-unavailable" | "component-unavailable" | "stale" | "expired" | "already-consumed" | "cancelled" | "deadline-exceeded";

export interface DocumentOpenPlanErrorV1 {
  schema: "semio.hub.document-open-plan-error/v1";
  code: DocumentOpenPlanErrorCodeV1;
}
/** 🧯️ Exact maximum accepted bytes for one verified raw package descriptor — the trusted catalog's
 * own `TRUSTED_DESCRIPTOR_MAX_BYTES`, shared verbatim by every transport. */
export const DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;
//#endregion 🔖️Views
