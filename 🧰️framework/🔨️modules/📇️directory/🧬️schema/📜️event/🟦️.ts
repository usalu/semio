/** 📜️ Directory event log wire contract: the event bodies, the document index entry and the `DirectoryEvent` envelope. */

import { type ArtifactHash, type DocumentScope } from "../../../📄️document/🪪️scope/🟦️.ts";
import type { DirectoryActor, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, Hlc } from "../🟦️.ts";
import type { DocumentDescriptor } from "../👀️views/🟦️.ts";
import type { ArtifactRetention, PublishedArtifactCheckpoint } from "../🧊️artifact-checkpoints/🟦️.ts";

//#region 🔖️Event
export interface DirectoryEventUserCreated {
  kind: "user.created";
  userId: string;
  email: string;
  displayName: string;
}

export interface DirectoryEventSpaceCreated {
  kind: "space.created";
  spaceId: string;
  name: string;
  spaceKind: DirectorySpaceKind;
  visibility: DirectorySpaceVisibility;
  ownerUserId: string;
}

export interface DirectoryEventSpaceRenamed {
  kind: "space.renamed";
  spaceId: string;
  name: string;
}

export interface DirectoryEventSpaceVisibilityChanged {
  kind: "space.visibility-changed";
  spaceId: string;
  visibility: DirectorySpaceVisibility;
}

/** 🧊️ Atomically freezes the space and demotes every current Author membership to Spectator. */
export interface DirectoryEventSpaceArchived {
  kind: "space.archived";
  spaceId: string;
}

export interface DirectoryEventSpaceDeleted {
  kind: "space.deleted";
  spaceId: string;
}

export interface DirectoryEventMemberUpserted {
  kind: "member.upserted";
  spaceId: string;
  userId: string;
  role: DirectorySpaceRole;
}

export interface DirectoryEventMemberRemoved {
  kind: "member.removed";
  spaceId: string;
  userId: string;
}

export interface DirectoryEventInviteRedeemed {
  kind: "invite.redeemed";
  spaceId: string;
  userId: string;
  inviteId: string;
  role: DirectorySpaceRole;
}

export interface DirectoryEventDocumentAnnounced {
  kind: "document.announced";
  descriptor: DocumentDescriptor;
}

/** 🏷️ Directory-owned display metadata, not executable descriptor authority. */
export interface DocumentIndexEntryV1 {
  name: string;
  dialect: { artifactKind: string; standard: string; subset: string };
}

/** 🧾️ One descriptor-bound presentation row derived from ordered Directory events. */
export interface DirectoryIndexedDocumentViewV1 {
  descriptor: DocumentDescriptor;
  descriptorDigestV1: ArtifactHash;
  entry: DocumentIndexEntryV1;
  createdAtMs: number;
  createdBy: string;
}

export interface DirectoryEventDocumentIndexed {
  kind: "document.indexed";
  scope: DocumentScope;
  descriptorDigestV1: ArtifactHash;
  entry: DocumentIndexEntryV1;
}

/** 🌟 The dialect grammar's any-subset coordinate, mirrored from `io_schema::SubsetId::ANY` and from
 * this law's Rust twin (`📇️document-index-v1/🦀️.rs`). */
export const DOCUMENT_INDEX_ANY_SUBSET_V1 = "*";

/** 🛡️ Validates the bounded presentation shape without authorizing document execution.
 *
 * 🌟 `subset` additionally admits the ONE canonical any-subset coordinate (`*`): it is the dialect
 * grammar's own wildcard, the value every dialect that declares no narrowed subset carries —
 * including the `parentDialect` the Directory's own document-open admission returns — so rejecting it
 * drops the presentation row of practically every indexed document, and with it the whole event page
 * that row rides on. It stays a bounded, non-executable literal: exactly `*`, never a pattern
 * embedded in a longer identity. The Rust twin has always said so; this side had not.
 *
 * 🪢 `dialect.artifactKind` is the owning app's `Dialect` coordinate (an installed artifact coordinate),
 * NOT the manifest `ArtifactKindSpec.id` its descriptor carries (`2d.note`, `stdio.json`) — two id
 * spaces that coincide for `gis` alone — so it is bounded by its own canonical grammar and never
 * compared to the descriptor. The plugin segment is compared to nothing: `demonstrator` ships apps
 * whose `Dialect` belongs to another plugin. */
export function validDocumentIndexEntryV1(entry: DocumentIndexEntryV1): boolean {
  const identity = (value: unknown): boolean => typeof value === "string" && value.length <= 256 && /^[A-Za-z0-9][A-Za-z0-9._:/-]*$/u.test(value);
  const subsetIdentity = (value: unknown): boolean => value === DOCUMENT_INDEX_ANY_SUBSET_V1 || identity(value);
  const canonicalDialectArtifactKind = (value: unknown): boolean => typeof value === "string" && value.length <= 256 && /^s(\.[a-z0-9]+(-[a-z0-9]+)*){1,2}$/u.test(value);
  return typeof entry.name === "string" && entry.name.length > 0 && [...entry.name].length <= 128 && !entry.name.startsWith(" ") && !entry.name.endsWith(" ") && !/[\u0000-\u001f\u007f-\u009f\uD800-\uDFFF]/u.test(entry.name)
    && canonicalDialectArtifactKind(entry.dialect.artifactKind) && identity(entry.dialect.standard) && subsetIdentity(entry.dialect.subset);
}

export interface DirectoryEventArtifactCheckpointPublished {
  kind: "artifact.checkpoint-published";
  checkpoint: PublishedArtifactCheckpoint;
}

export interface DirectoryEventArtifactRetentionAdvanced {
  kind: "artifact.retention-advanced";
  retention: ArtifactRetention;
}

/** 🎚️ One preference change of ONE user (`schema` names the vocabulary, `mutation` is its canonical JSON text — the hub
 * never reads it). Only that user sees it, and only on the preference lane (`DIRECTORY_PREFERENCE_PAGE_PATH_V1`): the
 * default page and the Home fold never carry it, so a guest folding directory pages never meets a kind it does not
 * know (ticket 26/09/23 U5). */
export interface DirectoryEventUserPreferenceRecorded {
  kind: "user.preference-recorded";
  userId: string;
  schema: string;
  mutation: string;
}

/** 🎚️ The preference lane's page route: the principal's own `user.preference-recorded` events on the page machinery of
 * `/directory/event-page/v1`, nothing else. */
export const DIRECTORY_PREFERENCE_PAGE_PATH_V1 = "/directory/preference-page/v1";
export const USER_PREFERENCE_SCHEMA_ID_MAX_BYTES = 128;
export const USER_PREFERENCE_MUTATION_MAX_BYTES = 4096;

/** 🛡️ A preference vocabulary id (`os.config.ui-preferences.v1`) and one JSON object text within its bound — the Rust
 * twin's `valid_user_preference_record_v1`. The hub never reads the object; the vocabulary's own clients do. */

export type DirectoryEventBody =
  | DirectoryEventUserPreferenceRecorded
  | DirectoryEventUserCreated
  | DirectoryEventSpaceCreated
  | DirectoryEventSpaceRenamed
  | DirectoryEventSpaceVisibilityChanged
  | DirectoryEventSpaceArchived
  | DirectoryEventSpaceDeleted
  | DirectoryEventMemberUpserted
  | DirectoryEventMemberRemoved
  | DirectoryEventInviteRedeemed
  | DirectoryEventDocumentAnnounced
  | DirectoryEventDocumentIndexed
  | DirectoryEventArtifactCheckpointPublished
  | DirectoryEventArtifactRetentionAdvanced;

/** 📜️ One persisted, backend-assigned directory event — `seq` is dense and 1-based. */
export interface DirectoryEvent {
  seq: number;
  id: string;
  hlc: Hlc;
  actor: DirectoryActor;
  spaceId?: string;
  userId?: string;
  body: DirectoryEventBody;
  recordedAtMs: number;
}
//#endregion 🔖️Event
