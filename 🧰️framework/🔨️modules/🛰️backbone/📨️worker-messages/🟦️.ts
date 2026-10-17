/** 📨️ Backbone worker messages: the request, response and bootstrap event types of the backbone worker protocol. */

import { parseInstalledServiceContributionsV1, type InstalledServiceOperationV1, type InstalledServiceStatusV1, type InstalledServiceHistoryStatusV1 } from "../../💡️inference/🔌️service/🟦️.ts";
import type { ArtifactDialect } from "@semio-tech/framework";
import type { DirectoryCommand, DirectoryEvent, DirectoryStreamMessage, DocumentScope } from "../../📇️directory/🟦️.ts";
import { type DocumentCheckInStatusV1 } from "../../📄️document/📌️check-in/🟦️.ts";
import { type InferencePortClosedV1, type InferencePortOpeningResultV1 } from "../../💡️inference/🚪️opening/🟦️.ts";
import type { DirectoryCommandErrorCodeV1, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1 } from "../../📇️directory/🧬️schema/🎫️command/🟦️.ts";
import type { DocumentExecutionTargetProgressV1, DocumentExecutionTargetStatusCodeV1 } from "../../📇️directory/🧬️schema/🎯️execution-target/🟦️.ts";
import { type BrowserActorUiPatchOfferV1, type BrowserActorUiPatchResultV1 } from "../../🌐️browser-bundle/🩹️patch-handoff/🟦️.ts";
import { type BrowserActorActionRequestV1, type BrowserActorActionResultV1 } from "../../🌐️browser-bundle/🎯️action-handoff/🟦️.ts";
import { type BrowserActorViewStateRequest } from "../../🌐️browser-bundle/🪟️view-context/🟦️.ts";
import { type DirectoryEventPageAckV1 } from "../../📞️directory-client/🟦️.ts";
import type { ArtifactActorConfig, ArtifactActorMsg, ArtifactEvent, BrowserActorUiMountedV1 } from "../🎭️artifact-actor/🟦️.ts";

//#region 🔖️BackboneWorkerProtocol
//#region 🔖️EnvelopeCodec
/** 🏛️ Closed lifecycle of the one shell-owned retained space-administration operation.
 * `receipt` is only ever reached through an exact accepted server receipt; every terminal phase
 * has already erased the page, the receipt, and any invite capability. */
export type DirectoryAdministrationPhaseV1 =
  | "loading"
  | "ready"
  | "submitting"
  | "receipt"
  | "refreshing"
  | "deleted"
  | "cancelled"
  | "denied"
  | "stale"
  | "failed";

/** 🗂️ The one independently paged administration window a cursor may advance. */
export type DirectoryAdministrationSectionV1 = "members" | "invites" | "documents";

/** 📋️ Renderer-visible state of the worker-retained invite capability. */
export type DirectoryAdministrationInviteCapabilityStatusV1 = "available" | "copying" | "failed";

/** 🌱️ One server-owned artifact creation's durable, non-CRUD lifecycle. Only `ready` may
 * disclose the server-minted document tuple that the Shell can pass to its ordinary open path. */
export type SpaceArtifactCreationPhaseV1 = "accepted" | "preparing" | "ready" | "indeterminate" | "failed" | "cancelled";

/** 🔐️ The exact public artifact identity produced by a ready creation saga. Catalog authority,
 * descriptors, grants and filesystem details remain server-private and are re-resolved by open-plan. */
export type SpaceArtifactCreationReadyV1 = Readonly<{
  artifactId: string;
  kindId: string;
  artifactSchema: string;
  parentDialect: ArtifactDialect;
}>;

/** 📡️ Renderer-visible status for one exact request id and Space. */
export type SpaceArtifactCreationStatusV1 = Readonly<{
  kind: "space-artifact-creation-status";
  requestId: string;
  spaceId: string;
  catalogGenerationId: string;
  phase: SpaceArtifactCreationPhaseV1;
  ready?: SpaceArtifactCreationReadyV1;
}>;

/** 🔄️ Local disposition for an ambiguous initial POST conflict. It requests a fresh selected
 * catalog without asserting a Hub mismatch or retrying the completed creation request. */
export type SpaceArtifactCreationCatalogRefreshRequiredV1 = Readonly<{
  kind: "space-artifact-creation-catalog-refresh-required";
  requestId: string;
  spaceId: string;
  catalogGenerationId: string;
}>;

/** 🗣️ One selected descriptor's exact bilingual creation presentation. */
export type SpaceArtifactCreationKindV1 = Readonly<{
  kindId: string;
  schema: string;
  dialect: ArtifactDialect;
  label: Readonly<{ en: string; de: string }>;
}>;

/** 🗂️ Current trusted creation choices for one authenticated Space. */
export type SpaceArtifactCreationCatalogV1 = Readonly<{
  kind: "space-artifact-creation-catalog";
  clientInstanceId: string;
  spaceId: string;
  catalogGenerationId: string;
  kinds: readonly SpaceArtifactCreationKindV1[];
}>;

export type SpaceArtifactCreationCatalogPhaseV1 = "loading" | "ready" | "unavailable";

/** 🗂️ Exact mounted-Space presentation state for the selected creation catalog. */
export type SpaceArtifactCreationCatalogStatusV1 = Readonly<{
  kind: "space-artifact-creation-catalog-status";
  clientInstanceId: string;
  spaceId: string;
  phase: SpaceArtifactCreationCatalogPhaseV1;
}>;

/** 📤️ Main thread → `🏪️store/👷️worker/🟦️.ts` messages (structured clone or {@link BackboneWorkerWireMessage}).
 * The `directory-*` kinds (contract-freeze §C6) are the shell's ONLY way to reach the directory hub
 * — plugin surfaces never talk to the network, and the shell never opens a directory socket on the
 * UI thread; see `🏪️store/👷️worker/🟦️.ts`'s `🔖️Directory` region. */
export type BackboneWorkerRequest =
  | (BrowserActorActionRequestV1 & { readonly clientInstanceId: string })
  | BrowserActorViewStateRequest
  | ({ readonly kind: "open"; readonly clientInstanceId?: string } & ArtifactActorConfig)
  | { readonly kind: "close"; readonly documentId: string; readonly spaceId?: string; readonly clientInstanceId?: string }
  | { readonly kind: "send"; readonly documentId: string; readonly spaceId?: string; readonly clientInstanceId?: string; readonly message: ArtifactActorMsg }
  | { readonly kind: "directory-open"; readonly baseUrl: string; readonly since: number }
  | { readonly kind: "directory-bootstrap-open"; readonly baseUrl: string; readonly after: number; readonly bootstrapEpoch: number }
  | ({ readonly kind: "directory-bootstrap-ack" } & DirectoryEventPageAckV1)
  | { readonly kind: "directory-bootstrap-reject"; readonly bootstrapEpoch: number; readonly receiptSha256: string }
  | { readonly kind: "directory-bootstrap-close"; readonly bootstrapEpoch: number }
  /** 🌐️ The signed-in user's preference lane (`DIRECTORY_PREFERENCE_PAGE_PATH_V1`): pages from `after`, then a wake on
   * every directory head that passes the lane's frontier. Recording a change is a `record-user-preference` directory
   * command whose `requestId` is the change's own, so a resend is the same command. */
  | { readonly kind: "preference-lane-open"; readonly baseUrl: string; readonly userId: string; readonly after: number }
  | { readonly kind: "preference-lane-record"; readonly requestId: string; readonly schema: string; readonly mutation: string }
  | { readonly kind: "preference-lane-close" }
  | { readonly kind: "directory-scope-open"; readonly baseUrl: string; readonly scope: DocumentScope; readonly since: number }
  | { readonly kind: "directory-scope-close"; readonly scope: DocumentScope }
  /** 📇️ A mounted space index's directory lane: its space's full event history (sealed pages from the first event),
   * then every live event of that space. The space index is the shell's projection of a space's directory, not a hub
   * document, so it has no document scope and no document-scoped stream. */
  | { readonly kind: "directory-space-open"; readonly baseUrl: string; readonly spaceId: string }
  | { readonly kind: "directory-space-close"; readonly spaceId: string }
  | { readonly kind: "directory-command"; readonly requestId: string; readonly command: DirectoryCommand }
  | { readonly kind: "directory-command-cancel"; readonly requestId: string }
  | { readonly kind: "space-artifact-creation-catalog-open"; readonly clientInstanceId: string; readonly spaceId: string }
  | { readonly kind: "space-artifact-create"; readonly requestId: string; readonly spaceId: string; readonly expectedCatalogGenerationId: string; readonly kindId: string; readonly name: string }
  | { readonly kind: "space-artifact-create-cancel"; readonly requestId: string; readonly spaceId: string }
  /** 📌️ Hub Check In of one mounted hub document at the head its socket had acknowledged once every
   * local edit is acknowledged; the worker names the head, the host never does. */
  | { readonly kind: "document-check-in"; readonly requestId: string; readonly clientInstanceId: string; readonly scope: DocumentScope }
  | { readonly kind: "document-check-in-cancel"; readonly requestId: string; readonly scope: DocumentScope }
  | { readonly kind: "directory-administration-open"; readonly operationEpoch: number; readonly spaceId: string }
  | { readonly kind: "directory-administration-refresh"; readonly operationEpoch: number; readonly cursor?: string }
  | { readonly kind: "directory-administration-submit"; readonly operationEpoch: number; readonly requestId: string; readonly command: DirectoryCommand }
  | { readonly kind: "directory-administration-capability-request"; readonly operationEpoch: number }
  | { readonly kind: "directory-administration-capability-result"; readonly operationEpoch: number; readonly transferEpoch: number; readonly copied: boolean }
  | { readonly kind: "directory-administration-close"; readonly operationEpoch: number }
  | { readonly kind: "directory-close" }
  | ReturnType<typeof parseInstalledServiceContributionsV1>
  | (InstalledServiceOperationV1 & { readonly kind: "service-operation" })
  | (BrowserActorUiPatchResultV1 & { readonly clientInstanceId: string });

/** 🛰️ Worker-local P2-C recovery lifecycle. These are not persisted artifact events: they describe
 * one bounded public bootstrap transfer and therefore remain explicit top-level worker responses. */
export type ArtifactBootstrapWorkerEvent =
  | {
      readonly kind: "artifact-bootstrap-progress";
      readonly documentId: string;
      readonly clientInstanceId: string;
      readonly receivedBytes: number;
      readonly totalBytes: number;
      readonly receivedChunks: number;
      readonly totalChunks: number;
      readonly scope?: DocumentScope;
    }
  | {
      readonly kind: "artifact-bootstrap-failed";
      readonly documentId: string;
      readonly clientInstanceId: string;
      readonly code: "cancelled" | "deadline-exceeded" | "invalid-bootstrap" | "transport-failure" | "recovery-exhausted";
      readonly message: string;
      readonly retryable: boolean;
      readonly scope?: DocumentScope;
    }
  | {
      readonly kind: "artifact-rebootstrap-required";
      readonly documentId: string;
      readonly clientInstanceId: string;
      readonly message: string;
      readonly retryable: true;
      readonly scope?: DocumentScope;
    };

/** 📥️ `🏪️store/👷️worker/🟦️.ts` → main thread messages. `directory-status.pendingCommands` is the
 * bounded, in-memory offline queue's length (contract-freeze §C6 "commands queue... and flush on
 * reconnect"). */
export type BackboneWorkerResponse =
  | (BrowserActorActionResultV1 & { readonly clientInstanceId: string })
  | { readonly kind: "event"; readonly documentId: string; readonly clientInstanceId: string; readonly event: ArtifactEvent; readonly scope?: DocumentScope; readonly verifiedSurfaceId?: string }
  | ArtifactBootstrapWorkerEvent
  | { readonly kind: "ready" }
  | { readonly kind: "directory-message"; readonly message: DirectoryStreamMessage }
  | ({ readonly kind: "directory-event-page"; readonly canonicalJson: string } & DirectoryEventPageAckV1 & { readonly afterSeqExclusive: number; readonly hasMore: boolean })
  | { readonly kind: "preference-lane-page"; readonly userId: string; readonly afterSeqExclusive: number; readonly throughSeqInclusive: number; readonly hasMore: boolean; readonly events: readonly { readonly seq: number; readonly schema: string; readonly mutation: string }[] }
  | { readonly kind: "preference-lane-recorded"; readonly requestId: string; readonly outcome: string }
  | { readonly kind: "preference-lane-failed"; readonly requestId: string | null; readonly code: string; readonly retryable: boolean }
  | { readonly kind: "directory-bootstrap-failed"; readonly bootstrapEpoch: number; readonly code: "unauthorized" | "cancelled" | "transport" | "invalid-page"; readonly retryable: boolean }
  | { readonly kind: "directory-scope-revoked"; readonly scope: DocumentScope }
  /** 📇️ Events of one space for its mounted index lane, in ascending `seq` (history first, then live). */
  | { readonly kind: "directory-space-events"; readonly spaceId: string; readonly events: readonly DirectoryEvent[] }
  | { readonly kind: "directory-command-receipt"; readonly requestId: string; readonly receipt: DirectoryCommandReceiptV1 }
  | { readonly kind: "directory-command-failed"; readonly requestId: string; readonly code: DirectoryCommandErrorCodeV1 }
  | SpaceArtifactCreationCatalogV1
  | SpaceArtifactCreationCatalogStatusV1
  | SpaceArtifactCreationStatusV1
  | SpaceArtifactCreationCatalogRefreshRequiredV1
  | { readonly kind: "socket-actor"; readonly documentId: string; readonly clientInstanceId: string; readonly scope?: DocumentScope; readonly actorId: string }
  /** 📌️ One hub Check In status, exactly as the hub answered it (or a local `failed` when the
   * document had no acknowledged head to name). */
  | { readonly kind: "document-check-in-status"; readonly clientInstanceId: string; readonly scope: DocumentScope; readonly status: DocumentCheckInStatusV1 }
  | { readonly kind: "socket-actor-failed"; readonly documentId: string; readonly clientInstanceId: string; readonly scope?: DocumentScope; readonly code: "installed-target-unavailable" | "session-mismatch" }
  /** 🪪️ Bounded execution-target install status for the React host's localized live region. It
   * carries a status code and byte counters only — never bytes, an origin, a path, a module URL, a
   * receipt, a grant or a digest. `diagnostic` is the one exception and never reaches the localized
   * text: it is the bounded failure line the owner itself named, surfaced beside the notice so a
   * terminal outcome is readable instead of anonymous. */
  | { readonly kind: "execution-target-status"; readonly documentId: string; readonly clientInstanceId: string; readonly spaceId: string; readonly scope?: DocumentScope; readonly code: DocumentExecutionTargetStatusCodeV1; readonly progress?: DocumentExecutionTargetProgressV1; readonly diagnostic?: string }
  /** 🏛️ The complete renderer-visible administration state. `canonicalJson` is the exact page the
   * hub sealed; `inviteCapabilityPending` says a one-shot invite token remains held by the worker
   * until exact clipboard success. No session identity, bearer, or invite token ever appears here. */
  | {
      readonly kind: "directory-administration-state";
      readonly operationEpoch: number;
      readonly spaceId: string;
      readonly phase: DirectoryAdministrationPhaseV1;
      readonly canonicalJson?: string;
      readonly receiptSha256?: string;
      readonly outcome?: DirectoryCommandOutcomeV1;
      readonly code?: DirectoryCommandErrorCodeV1;
      readonly inviteCapabilityPending?: boolean;
      readonly inviteCapabilityStatus?: DirectoryAdministrationInviteCapabilityStatusV1;
    }
  /** 🎁️ One operation-bound clipboard offer. The worker retains the capability until the renderer
   * reports that this exact transfer epoch was copied. */
  | { readonly kind: "directory-administration-capability"; readonly operationEpoch: number; readonly transferEpoch: number; readonly inviteToken: string }
  /** 🛡️ A duplicate or stale request/result was rejected without redisclosing the capability. */
  | { readonly kind: "directory-administration-capability-rejected"; readonly operationEpoch: number; readonly transferEpoch?: number; readonly code: "capacity" | "already-settled" | "mismatch" }
  /** 💡️ The complete renderer-visible state of one document's inference port. It carries the
   * phase, the server's own job id, the bounded progress cursor and the hash the server published —
   * never a receipt, bearer, origin, path, base pack, proposal body or user identity. */
  | { readonly kind: "inference-port-status"; readonly operationEpoch: number; readonly scope: DocumentScope; readonly status: InstalledServiceStatusV1 }
  | InferencePortOpeningResultV1
  | InferencePortClosedV1
  | { readonly owner: string; readonly serviceId: string; readonly kind: "inference-history-status"; readonly historyEpoch: number; readonly clientInstanceId: string; readonly scope: DocumentScope; readonly status: InstalledServiceHistoryStatusV1 }
  /** 🔬️ Public checkpoint/frontier identity for one exact actor UI revision, emitted only after
   * the guest accepts the Shell's transactional patch acknowledgement. It carries no plan, grant,
   * credential, receipt or action. */
  | BrowserActorUiMountedV1
  | (BrowserActorUiPatchOfferV1 & { readonly clientInstanceId: string })
  | { readonly kind: "directory-status"; readonly pendingCommands: number };
//#endregion 🔖️EnvelopeCodec
//#endregion 🔖️BackboneWorkerProtocol
