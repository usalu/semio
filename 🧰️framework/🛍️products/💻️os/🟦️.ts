// #region Header
/**
 * 🖥️ `@semio-tech/framework-os` — JS sync/backbone protocol surface (backbone URIs, document
 * envelopes, `🏪️store/👷️worker/🟦️.ts` request/response wire types, `PersistenceBinding`/`MutationEnvelope`,
 * {@link buildFrameworkSyncUtilities}) consumed by `framework/os/renderer/js/react/index.tsx` and
 * `framework/os/dev/script.ts`. The OS kernel's *stateful* logic (operation application, program
 * registry) is Rust/wasm-only, hosted by the s-plugin wasm — this file is not a JS port of that. The
 * one exception is {@link planWorkflow}: a pure, side-effect-free scheduling function has no state
 * to keep in sync with a live wasm host, so it's hand-mirrored here against the Rust `plan_workflow`
 * (`framework/os/core/rs/lib.rs`) with shared fixtures (`framework/os/core/fixtures/`)
 * asserting parity.
 */
// #endregion Header

import { parseDirectorySpaceAdministrationPageV1 } from "../../📦️packages/🟦️typescript/📇️directory/🚪️io/📝️text/🟦️.ts";
import { emptyDirectoryReadModel, foldAll } from "../../📦️packages/🟦️typescript/📇️directory/🟦️.ts";
import {
  clonePackValue,
  decodeInvocationResultPacks,
  decodeMeshPackBody,
  decodeMeshPackChunks,
  decodePackValue,
  decodeScenePackValue,
  encodePackValue,
  INVOCATION_RESULT_PACK_MAXIMUM_BYTES,
  isPackByteVector,
  isPackInteger,
  packInt,
  packUInt,
  packValueToExactJson
} from "../../📦️packages/🟦️typescript/🎁️packs/🟦️.ts";
import {
  COMMAND_REJECTION_CODES_V1,
  commandAckOutcomeOfValueV1,
  decodeDocumentArchiveBytes,
  decodeHubRejectionMessagesV1,
  encodeDocumentArchiveBytes,
  hubCommandRejectionV1,
  localCommandRejectionV1
} from "../../📦️packages/🟦️typescript/🪢️wire-contracts/🟦️.ts";
import {
  BACKBONE_ENVELOPE_RETRY_WINDOW_MS,
  BACKBONE_WORKER_WIRE_MAGIC,
  backboneKindFromUri,
  buildFileBackboneUri,
  buildFolderBackboneUri,
  buildFrameworkSyncUtilities,
  buildRemoteBackboneUri,
  decodeBackboneMessage,
  decodeBackboneWorkerRequest,
  decodeBackboneWorkerResponse,
  decodeDocumentPackBytes,
  decodeDocumentPackSnapshot,
  encodeBackboneMessage,
  encodeBackboneWorkerRequest,
  encodeBackboneWorkerResponse,
  encodeDocumentPackBundle,
  encodeDocumentPackBytes,
  parseDocumentBackboneMessage,
  parseInboundDocumentBackboneMessage,
  parseRemoteBackboneUri,
  planWorkflow,
  readBackboneEnvelope,
  writeBackboneEnvelope
} from "../../📦️packages/🟦️typescript/🛰️backbone/🟦️.ts";
import {
  admitGuestChannelVersion,
  APP_CHANNEL_VERSION,
  AppChannelClient,
  AppChannelRequestSequence,
  CHANNEL_MISMATCH_CODE,
  decodeAppCommand,
  decodeAppFrame,
  decodeConflictsFromWire,
  decodeDispatchReportFromWire,
  decodeMergeReportFromWire,
  encodeAppCommand,
  encodeAppFrame,
  faultMessages
} from "../../📦️packages/🟦️typescript/🧾️app-channel/🟦️.ts";
import {
  admitRemoteEnvelopes,
  DIRECTORY_HTTP_TIMEOUT_MS,
  DirectoryClient,
  DOCUMENT_LINK_ACCESS_REFUSED_STATUSES,
  DOCUMENT_LINK_SHORTAGE_POLICY,
  DOCUMENT_LINK_STATUS_TEXT,
  documentLinkAdmitsLocalEdits,
  documentLinkExpiresAtMs,
  documentLinkOpened,
  documentLinkStatus,
  documentLinkTransition,
  HUB_HEALTHY_RESET_MS,
  HUB_RECONNECT_MAX_MS,
  HUB_RECONNECT_MIN_MS,
  HUB_TRANSIENT_APPLY_REFUSAL_CODE,
  hubTransientApplyRefusalV1,
  noteAuthoredEnvelopeIds
} from "../../📦️packages/🟦️typescript/📞️directory-client/🟦️.ts";

export {
  type ActionWire,
  asPackUInt32,
  asPackUIntSafe,
  clonePackValue,
  decodeActionWire,
  decodeInvocationResultPacks,
  decodeMeshPackBody,
  decodeMeshPackChunks,
  decodeMutationEnvelopesPack,
  decodePackValue,
  decodePackWire,
  decodeScenePackField,
  decodeScenePackValue,
  describePackWireValue,
  encodeActionWire,
  encodeMutationEnvelopesPack,
  encodePackValue,
  INVOCATION_RESULT_PACK_MAXIMUM_BYTES,
  isPackByteVector,
  isPackInteger,
  isPackMap,
  type MeshPack,
  packByteCompare,
  packInt,
  type PackInteger,
  packUInt,
  packUIntSafeOrNull,
  type PackValue,
  packValueFromBase64,
  packValueToBase64,
  packValueToExactJson,
  packWireNatural,
  viewContextWireValue
} from "../../📦️packages/🟦️typescript/🎁️packs/🟦️.ts";

export {
  type AppCommandValue,
  type AppDocumentIdentity,
  type AppFrameValue,
  BACKBONE_GENESIS_MAXIMUM_BYTES,
  BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES,
  type ChildHeadPackEntry,
  type ChildPackEntry,
  COMMAND_REJECTION_CODES_V1,
  type CommandAckOutcome,
  commandAckOutcomeOfValueV1,
  type CommandRejectionCodeV1,
  type CommandRejectionDetailV1,
  type CommandRejectionV1,
  decodeDocumentArchiveBytes,
  decodeHubRejectionMessagesV1,
  DOCUMENT_ARCHIVE_MAXIMUM_BYTES,
  DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS,
  type DocumentArchiveArtifactRef,
  type DocumentArchiveLoadState,
  type DocumentArchiveLoadStatus,
  type DocumentArchiveOwnerRef,
  type DocumentArchivePack,
  encodeDocumentArchiveBytes,
  hubCommandRejectionV1,
  localCommandRejectionV1,
  type MediaExportHandle,
  type MediaExportState,
  type MediaExportStatus,
  type OperationCompletionV1,
  type OwnedDocumentMemberPackEntry,
  type WindowConfigPackEntry
} from "../../📦️packages/🟦️typescript/🪢️wire-contracts/🟦️.ts";

export {
  type ArtifactActorConfig,
  type ArtifactActorMsg,
  artifactBackboneRef,
  type ArtifactBackboneRef,
  type ArtifactBootstrapWorkerEvent,
  type ArtifactEvent,
  type ArtifactSyncStatus,
  BACKBONE_ENDPOINT_PATH,
  BACKBONE_ENVELOPE_RETRY_WINDOW_MS,
  BACKBONE_WORKER_WIRE_MAGIC,
  type BackboneKind,
  backboneKindFromUri,
  type BackboneWorkerRequest,
  type BackboneWorkerResponse,
  type BackboneWorkerWireMessage,
  type BinaryBackboneMessage,
  bindingsDataClass,
  BLOB_ENDPOINT_PATH,
  type BrowserActorUiMountedV1,
  buildFileBackboneUri,
  buildFolderBackboneUri,
  buildFrameworkSyncUtilities,
  buildRemoteBackboneUri,
  type ClassifiedWireLane,
  decodeBackboneMessage,
  decodeBackboneWorkerRequest,
  decodeBackboneWorkerResponse,
  decodeDocumentPackBytes,
  decodeDocumentPackSnapshot,
  DESKTOP_WINDOW_CONTROL_CHANNELS,
  type DesktopWindowControls,
  desktopWindowControlsBridge,
  DEV_STREAM_ROUTES,
  DIRECTORY_SPACE_EVENTS_MAX_PER_MESSAGE,
  type DirectoryAdministrationInviteCapabilityStatusV1,
  type DirectoryAdministrationPhaseV1,
  type DirectoryAdministrationSectionV1,
  type DocumentBackboneMessage,
  documentRuntimeKeyV1,
  type DocumentRuntimeScopeV1,
  encodeBackboneMessage,
  encodeBackboneWorkerRequest,
  encodeBackboneWorkerResponse,
  encodeDocumentPackBundle,
  encodeDocumentPackBytes,
  ephemeralLocalPersistenceBinding,
  ephemeralSharedPersistenceBinding,
  folderPersistenceBinding,
  FRAMEWORK_SYNC_CONTROLLER_ID,
  type FrameworkSyncUtilityLeaf,
  hubPersistenceBinding,
  type MediaContract,
  type OsMediaPort,
  type OsWorkflow,
  type OsWorkflowEdge,
  type OsWorkflowNode,
  parseDocumentBackboneMessage,
  parseInboundDocumentBackboneMessage,
  parseRemoteBackboneUri,
  parseSpaceArtifactCreationCatalogRefreshRequiredV1,
  parseSpaceArtifactCreationCatalogStatusV1,
  parseSpaceArtifactCreationCatalogV1,
  parseSpaceArtifactCreationStatusV1,
  type PersistenceBinding,
  type PersistenceDataClass,
  planWorkflow,
  readBackboneEnvelope,
  registerDesktopWindowControlHandlers,
  type RemoteState,
  resolveArtifactOpeningRelay,
  type ResolvedArtifactOpeningRelay,
  type SpaceArtifactCreationCatalogPhaseV1,
  type SpaceArtifactCreationCatalogRefreshRequiredV1,
  type SpaceArtifactCreationCatalogStatusV1,
  type SpaceArtifactCreationCatalogV1,
  type SpaceArtifactCreationKindV1,
  type SpaceArtifactCreationPhaseV1,
  type SpaceArtifactCreationReadyV1,
  type SpaceArtifactCreationStatusV1,
  STREAM_MUX_ANNOUNCEMENT,
  STREAM_MUX_PATH,
  type SyncConflict,
  wireLaneDataClass,
  type WorkflowDelivery,
  writeBackboneEnvelope
} from "../../📦️packages/🟦️typescript/🛰️backbone/🟦️.ts";

export {
  admitGuestChannelVersion,
  APP_CHANNEL_VERSION,
  AppChannelClient,
  type AppChannelHandle,
  type AppChannelOperationProgressV1,
  AppChannelRequestSequence,
  CHANNEL_MISMATCH_CODE,
  decodeAppCommand,
  decodeAppFrame,
  decodeConflictsFromWire,
  decodeDispatchReportFromWire,
  decodeFaultFromWire,
  decodeMergeReportFromWire,
  DOCUMENT_ARCHIVE_UNMERGEABLE_CODES,
  DocumentArchiveFaultError,
  type DocumentArchiveLoadAnswer,
  DocumentArchiveLoadHost,
  type DocumentArchiveLoadOutcome,
  type DocumentArchiveLoadStep,
  type DocumentArchiveMergeV1,
  encodeAppCommand,
  encodeAppFrame,
  faultDisplayMessage,
  faultMessages
} from "../../📦️packages/🟦️typescript/🧾️app-channel/🟦️.ts";

export {
  admitRemoteEnvelopes,
  type CanonicalDirectoryEventPageV1,
  type CanonicalDirectorySpaceAdministrationPageV1,
  createSocketGrantIssuerV1,
  DIRECTORY_HTTP_TIMEOUT_MS,
  type DirectoryAcknowledgedStream,
  DirectoryClient,
  DirectoryCommandError,
  type DirectoryEventPageAckV1,
  DirectoryHttpError,
  type DirectoryPreferencePageV1,
  type DirectoryRequestOptions,
  type DirectoryStream,
  DOCUMENT_LINK_ACCESS_REFUSED_STATUSES,
  DOCUMENT_LINK_SHORTAGE_POLICY,
  DOCUMENT_LINK_STATUS_TEXT,
  type DocumentLink,
  documentLinkAdmitsLocalEdits,
  documentLinkCeilingAtMs,
  type DocumentLinkEvent,
  documentLinkExpiresAtMs,
  documentLinkOpened,
  documentLinkRetryAtMs,
  type DocumentLinkShortagePolicy,
  documentLinkStatus,
  type DocumentLinkStatusCode,
  documentLinkTransition,
  type DocumentSocketGrantReceiptV1,
  HUB_HEALTHY_RESET_MS,
  HUB_RECONNECT_MAX_MS,
  HUB_RECONNECT_MIN_MS,
  HUB_SESSION_CAPABILITY_PATTERN_V1,
  HUB_TRANSIENT_APPLY_REFUSAL_CODE,
  type HubSessionPortRequestV1,
  type HubSessionPortResponseV1,
  hubTransientApplyRefusalV1,
  mediaAcceptFilterKinds,
  normalizeStdioFormatKind,
  noteAuthoredEnvelopeIds,
  parseDirectorySpaceListEntryV1,
  parseDocumentSocketGrantReceiptV1,
  parseHubSessionPortRequestV1,
  parseHubSessionPortResponseV1,
  parseSocketGrantReceiptV1,
  type SocketGrantIssuerV1,
  socketGrantProtocolsV1,
  type SocketGrantReceiptV1,
  type SocketGrantRequestPortV1
} from "../../📦️packages/🟦️typescript/📞️directory-client/🟦️.ts";

export type {RetainedCloneGrant} from "@semio-tech/value-ts/retained-clone/wire";
export { parseDirectorySessionAuthorityJsonV1 } from "../../📦️packages/🟦️typescript/📇️directory/🚪️io/📝️text/🪪️session-authority-v1/🟦️.ts";
export { directoryAdministrationCommandAllowedV1 } from "../../📦️packages/🟦️typescript/📇️directory/🚪️io/📝️text/🟦️.ts";
export { directoryCommandRequestJson } from "../../📦️packages/🟦️typescript/📇️directory/🚪️io/📝️text/🟦️.ts";
export { canonicalDirectoryCommandV1, directoryCommandSha256, parseDirectoryCommandReceiptV1, parseDirectoryCommandRequestV1, parseDirectoryCommandV1, sealDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1 } from "../../📦️packages/🟦️typescript/📇️directory/🚪️io/📝️text/🟦️.ts";
export { parseInstalledServiceContributionsV1, parseInstalledServiceOperationV1, parseInstalledServiceStatusV1, parseInstalledServiceHistoryStatusV1 } from "../../📦️packages/🟦️typescript/💡️inference/🔌️service/🟦️.ts";
export type { InstalledServiceOperationV1, InstalledServiceStatusV1, InstalledServiceHistoryStatusV1 } from "../../📦️packages/🟦️typescript/💡️inference/🔌️service/🟦️.ts";
export { type DirectorySessionAuthorityV1 } from "@semio-tech/framework/directory/schema/session-authority-v1";
export { artifactFrontierIsEditedForV1, artifactFrontierIsGenesisForV1 } from "../../📦️packages/🟦️typescript/📇️directory/🧬️schema/🟦️.ts";
export type { BrowserActorUiPatchOfferV1, BrowserActorUiPatchResultV1 } from "@semio-tech/framework/browser-bundle/patch-handoff";
//#region 🔖️BackboneWorkerProtocol
/** 🪪️ Hub documents compare a plan against the complete {@link DocumentExecutionTargetLeaseFieldsV1}
 * of a locally installed execution target — catalog generation, both scope ids, descriptor digest,
 * every package/component/descriptor digest with byte lengths, artifact, parent dialect, surface,
 * grant, checkpoint and revalidation. A non-`react` renderer target is admitted only when the
 * worker owns a live private lease that verified those exact bytes. */
export type { DirectoryCommandErrorCodeV1, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryCommandResultV1, DocumentExecutionTargetLeaseFieldsV1, DocumentExecutionTargetProgressV1, DocumentExecutionTargetStatusCodeV1 } from "../../📦️packages/🟦️typescript/📇️directory/🧬️schema/🟦️.ts";
export { DIRECTORY_COMMAND_OUTCOMES_V1, DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, DIRECTORY_COMMAND_REQUEST_MAX_BYTES, directoryCommandErrorIsTransient, parseDirectoryCommandOutcomeV1 } from "../../📦️packages/🟦️typescript/📇️directory/🧬️schema/🟦️.ts";
export { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, leaseFieldsFromPlanV1, parseDocumentExecutionTargetLeaseFieldsV1, sameLeaseFieldsV1 } from "./🔨️modules/📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
export { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, documentExecutionTargetStatusRoleV1 } from "../../📦️packages/🟦️typescript/📇️directory/🧬️schema/🟦️.ts";
//#endregion 🔖️BackboneWorkerProtocol
export type {
  AdminConnectionSnapshotV1,
  AdminIntentOutcomeV1,
  AdminIntentReceiptV1,
  AdminIntentResultV1,
  AdminIntentStateV1,
  AdminIntentV1,
  AdminOperationAuditPhaseV1,
  AdminOperationAuditV1,
  AdminOperationProgressV1,
  AdminOperationStatusV1,
  AdminPageV1,
  AdminRecordedConnectionV1,
  ArtifactBlobRef,
  ArtifactCheckpoint,
  ArtifactFrontier,
  ArtifactHash,
  ArtifactRetention,
  CheckpointId,
  ConnectionView,
  DirectoryActor,
  DirectoryActorKind,
  DirectoryCommand,
  DirectoryConnectionPhase,
  DirectoryEvent,
  DirectoryEventBody,
  DirectoryReadModel,
  DirectorySpace,
  DirectorySpaceAdministrationCapabilitiesV1,
  DirectorySpaceAdministrationInviteRowV1,
  DirectorySpaceAdministrationMemberRowV1,
  DirectorySpaceAdministrationPageV1,
  DirectorySpaceAdministrationSectionV1,
  DirectorySpaceAdministrationWindowV1,
  DirectorySpaceKind,
  DirectorySpaceListEntryV1,
  DirectorySpaceRole,
  DirectorySpaceVisibility,
  DirectoryStreamMessage,
  DocumentDescriptor,
  DocumentFrontier,
  DocumentOwner,
  DocumentScope,
  DocumentView,
  Hlc,
  InviteView,
  MemberSpaceViewV1,
  MemberView,
  PublicDocumentCatalogEntryV1,
  PublicSpaceViewV1,
  SpaceView,
  UserView,
} from "../../📦️packages/🟦️typescript/📇️directory/🟦️.ts";
export { decodeServerFrame, encodeServerFrame } from "../../🔨️modules/📡️replication/📦️packages/🟦️typescript/🟦️.ts";
export { emptyDirectoryReadModel, fold, foldAll, isDirectoryCommandKind, isDirectoryEventBodyKind, isDirectoryStreamMessageKind } from "../../📦️packages/🟦️typescript/📇️directory/🟦️.ts";
export { parseDirectorySpaceAdministrationPageV1 } from "../../📦️packages/🟦️typescript/📇️directory/🚪️io/📝️text/🟦️.ts";
//#region 🔖️HubSignIn
/** 🔐️ ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END slice AU2 — the browser-facing hub
 * sign-in contract (`POST /auth/sessions`, per that ticket's `📓️au1-hub-auth-sessions-and-rate-limit.md`
 * §1) and the end-user spaces surface contract, both pure and transport-free. Appended only. */
export { HUB_AUTH_ERROR_SCHEMA_V1, HUB_CONNECTION_BOOK_MAX_ENTRIES, HUB_CONNECTION_BOOK_SCHEMA_V1, HUB_CONNECTION_BOOK_STORAGE_KEY_V1, HUB_SESSION_MINT_PATH_V1, HUB_SESSION_MINT_REQUEST_MAX_BYTES, HUB_SESSION_MINT_RESPONSE_MAX_BYTES, HUB_SESSION_ME_PATH_V1, HUB_SESSION_SIGN_OUT_PATH_V1, HUB_SIGN_IN_DEVICE_INSTANCE_MAX_BYTES, HUB_SIGN_IN_EMAIL_MAX_BYTES, HUB_SIGN_IN_EMAIL_MIN_BYTES, HUB_SIGN_IN_PASSWORD_MAX_BYTES, HUB_SIGN_IN_PASSWORD_MIN_BYTES, HUB_SIGN_IN_RATE_LIMIT_MAX_SECONDS, HUB_SIGN_IN_REQUEST_SCHEMA_V1, HUB_SIGN_IN_TEXT_V1, HUB_SIGN_IN_TIMEOUT_MS, LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1, hubAuthErrorRetryAfterSecondsV1, hubConnectionIdForOriginV1, hubDeviceInstanceIdV1, hubRetryAfterSecondsV1, hubSessionAllowsLocalWorkV1, hubSessionInitialStateV1, hubSessionMintRequestJsonV1, hubSessionNeedsReauthenticationV1, hubSignInErrorFromStatusV1, hubSignInErrorTextV1, hubSignInTextV1, localBootstrapHubConnectionV1, parseHubConnectionBookV1, parseHubOriginV1, parseHubSessionMintResultV1, readHubConnectionBookV1, reduceHubSessionV1, removeHubConnectionV1, runHubSignInV1, runHubSignOutV1, selectHubConnectionV1, selectedHubConnectionV1, serializeHubConnectionBookV1, upsertHubConnectionV1, validHubSignInEmailV1, validHubSignInPasswordV1, writeHubConnectionBookV1 } from "../../📦️packages/🟦️typescript/📇️directory/🔐️sign-in/🟦️.ts";
export type {
  HubConnectionBookV1,
  HubConnectionKindV1,
  HubConnectionStorageV1,
  HubConnectionV1,
  HubSessionEventV1,
  HubSessionMintResultV1,
  HubSessionPhaseV1,
  HubSessionStateV1,
  HubSignInClientClassV1,
  HubSignInCredentialV1,
  HubSignInErrorCodeV1,
  HubSignInLocaleV1,
  HubSignInOutcomeV1,
  HubSignInResponseV1,
  HubSignInTransportV1,
} from "../../📦️packages/🟦️typescript/📇️directory/🔐️sign-in/🟦️.ts";
export { DIRECTORY_COMMANDS_PATH_V1, DIRECTORY_SPACES_PATH_V1, INVITE_LINK_FRAGMENT_V1, INVITE_TOKEN_MAX_BYTES, INVITE_TTL_CHOICES_SECS_V1, SPACE_NAME_MAX_BYTES, archiveSpaceCommandV1, createInviteCommandV1, createSpaceCommandV1, filterSpaceRowsV1, inviteLinkV1, inviteRedeemPathV1, inviteRedemptionErrorFromStatusV1, parseInviteTokenV1, spaceBrowserRowsUsableV1, spaceMemberPresenceV1, spaceRowInvitableV1, spaceRowWritableV1, spaceRowsV1 } from "../../📦️packages/🟦️typescript/📇️directory/🏘️spaces/🟦️.ts";
export type { InviteRedemptionErrorCodeV1, SpaceAccessV1, SpaceBrowserPhaseV1, SpaceMemberPresenceV1, SpaceRowV1 } from "../../📦️packages/🟦️typescript/📇️directory/🏘️spaces/🟦️.ts";
//#endregion 🔖️HubSignIn
//#region 🔖️HubFirstRun
/** 🎓️ ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END slice D2 (G12 ranked item #12) — the hub
 * first-run walkthrough contract: one `IntroductionDefinition` covering sign in → create or join a
 * space → invite, in both owned locales, plus the local-only seen flag. Pure, transport-free, and
 * state-independent apart from the entry step. Appended only. */
export { HUB_FIRST_RUN_ANCHORS_V1, HUB_FIRST_RUN_SEEN_VALUE_V1, HUB_FIRST_RUN_STAGES_V1, HUB_FIRST_RUN_STORAGE_KEY_V1, HUB_FIRST_RUN_TEXT_V1, hubFirstRunIntroductionV1, hubFirstRunSeenV1, hubFirstRunShouldStartV1, hubFirstRunStageV1, hubFirstRunStepIndexV1, hubFirstRunTextV1, markHubFirstRunSeenV1 } from "../../📦️packages/🟦️typescript/📇️directory/🎓️first-run/🟦️.ts";
export type { HubFirstRunLocaleV1, HubFirstRunStageV1, HubFirstRunStateV1 } from "../../📦️packages/🟦️typescript/📇️directory/🎓️first-run/🟦️.ts";
//#endregion 🔖️HubFirstRun

//#region 🔖️MeshPackCodec
if (import.meta.vitest) {
  const { registerTests5 } = await import("./🧪️tests/🧊️mesh-pack-decode/🟦️.ts");
  await registerTests5(import.meta.vitest, { decodeMeshPackBody, decodeMeshPackChunks }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🔖️MeshPackCodec

//#region 🔖️Directory

//#region 🔖️HubBinding
//#region 🔌️DocumentLinkShortage
if (import.meta.vitest) {
  const { registerDocumentLinkShortageTests } = await import("./🧪️tests/🔌️document-link-shortage/🟦️.ts");
  await registerDocumentLinkShortageTests(import.meta.vitest, { DOCUMENT_LINK_ACCESS_REFUSED_STATUSES, DOCUMENT_LINK_SHORTAGE_POLICY, DOCUMENT_LINK_STATUS_TEXT, documentLinkAdmitsLocalEdits, documentLinkExpiresAtMs, documentLinkOpened, documentLinkStatus, documentLinkTransition });
}
//#endregion 🔌️DocumentLinkShortage

//#region 🔁️DocumentEchoSuppression
if (import.meta.vitest) {
  const { registerDocumentEchoSuppressionTests } = await import("./🧪️tests/🔁️document-echo-suppression/🟦️.ts");
  await registerDocumentEchoSuppressionTests(import.meta.vitest, { admitRemoteEnvelopes, noteAuthoredEnvelopeIds });
}
//#endregion 🔁️DocumentEchoSuppression

//#region ⏳️TransientApplyRefusal
if (import.meta.vitest) {
  const { registerTransientApplyRefusalTests } = await import("./🧪️tests/⏳️transient-apply-refusal/🟦️.ts");
  await registerTransientApplyRefusalTests(import.meta.vitest, { HUB_TRANSIENT_APPLY_REFUSAL_CODE, hubTransientApplyRefusalV1 });
}
//#endregion ⏳️TransientApplyRefusal

//#region 🚫️CommandRejection
if (import.meta.vitest) {
  const { registerCommandRejectionTests } = await import("./🔨️modules/🏪️store/🔄️sync/🧪️tests/🧪️command-rejection/🟦️.ts");
  await registerCommandRejectionTests(import.meta.vitest, { COMMAND_REJECTION_CODES_V1, commandAckOutcomeOfValueV1, decodeBackboneWorkerResponse, decodeHubRejectionMessagesV1, encodePackValue, hubCommandRejectionV1, localCommandRejectionV1, packUInt, BACKBONE_WORKER_WIRE_MAGIC });
}
//#endregion 🚫️CommandRejection

//#region 🔑️DirectoryAccessChanged
if (import.meta.vitest) {
  const { registerDirectoryAccessChangedTests } = await import("./🧪️tests/🔑️directory-access-changed/🟦️.ts");
  await registerDirectoryAccessChangedTests(import.meta.vitest);
}
//#endregion 🔑️DirectoryAccessChanged

//#region 📤️OutboundAnnouncement
if (import.meta.vitest) {
  const { registerOutboundAnnouncementTests } = await import("./🧪️tests/📤️outbound-announcement/🟦️.ts");
  await registerOutboundAnnouncementTests(import.meta.vitest);
}
//#endregion 📤️OutboundAnnouncement

//#region ⚔️ConcurrentWrite
if (import.meta.vitest) {
  const { registerConcurrentWriteTests } = await import("./🧪️tests/⚔️concurrent-write/🟦️.ts");
  await registerConcurrentWriteTests(import.meta.vitest);
}
//#endregion ⚔️ConcurrentWrite

//#endregion 🔖️HubBinding
//#endregion 🔖️Directory
