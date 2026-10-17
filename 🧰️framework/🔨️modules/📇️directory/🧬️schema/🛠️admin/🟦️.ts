/** 🛠️ Directory administration intents: the intent union, its outcomes, receipts, progress, recorded connections and the operation audit. */

import { type DocumentScope } from "../../../📄️document/🪪️scope/🟦️.ts";
import type { DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility } from "../🟦️.ts";

//#region 🔖️Admin
export type AdminIntentV1 =
  | { kind: "create-space"; requestId: string; name: string; spaceKind: DirectorySpaceKind; visibility: DirectorySpaceVisibility }
  | { kind: "rename-space"; requestId: string; spaceId: string; name: string }
  | { kind: "set-space-visibility"; requestId: string; spaceId: string; visibility: DirectorySpaceVisibility }
  | { kind: "archive-space"; requestId: string; spaceId: string }
  | { kind: "delete-space"; requestId: string; spaceId: string }
  | { kind: "upsert-space-member"; requestId: string; spaceId: string; email: string; role: DirectorySpaceRole }
  | { kind: "remove-space-member"; requestId: string; spaceId: string; userId: string }
  | { kind: "create-space-invite"; requestId: string; spaceId: string; role: DirectorySpaceRole; ttlSecs: number }
  | { kind: "revoke-space-invite"; requestId: string; spaceId: string; inviteId: string }
  | { kind: "issue-document-share"; requestId: string; scope: DocumentScope; ttlSecs: number }
  | { kind: "revoke-document-share"; requestId: string; scope: DocumentScope; shareId: string; reasonCode: string }
  | { kind: "revoke-user-sessions"; requestId: string; userId: string; reasonCode: string }
  | { kind: "kick-connection"; requestId: string; syncSessionId: string; reasonCode: string }
  | { kind: "rebuild-directory-projections"; requestId: string; expectedHeadSeq: number };

export type AdminIntentStateV1 = "succeeded" | "accepted" | "indeterminate" | "failed" | "cancelled";

export interface AdminIntentOutcomeV1 {
  code: string;
  durable: boolean;
  kickAttempted?: number;
  kickSignalled?: number;
}

export interface AdminIntentResultV1 {
  inviteToken?: string;
  shareToken?: string;
}

export interface AdminIntentReceiptV1 {
  operationId: string;
  correlationId: string;
  state: AdminIntentStateV1;
  eventSeqFirst?: number;
  eventSeqLast?: number;
  result?: AdminIntentResultV1;
  outcome: AdminIntentOutcomeV1;
}

export interface AdminOperationProgressV1 {
  completedEvents: number;
  totalEvents: number;
  cancelRequested: boolean;
}

export interface AdminOperationStatusV1 {
  receipt: AdminIntentReceiptV1;
  progress?: AdminOperationProgressV1;
}

export interface AdminPageV1<T> {
  rows: T[];
  nextCursor?: string;
  observedAtMs: number;
}

export interface AdminRecordedConnectionV1 {
  syncSessionId: string;
  scope: DocumentScope;
  authenticatedUserId?: string;
  email?: string;
  role?: DirectorySpaceRole;
  connectedAtMs: number;
  source: "recorded-sync-session";
}

export interface AdminConnectionSnapshotV1 extends AdminPageV1<AdminRecordedConnectionV1> {
  source: "recorded-sync-sessions";
  headSeq: number;
}

export type AdminOperationAuditPhaseV1 = "accepted" | "succeeded" | "failed" | "cancelled";

export interface AdminOperationAuditV1 {
  sequence: number;
  operationId: string;
  occurredAtMs: number;
  phase: AdminOperationAuditPhaseV1;
  intentKind: string;
  targetKind: string;
  targetId: string;
  principalUserId: string;
  principalSessionId: string;
  principalGeneration: number;
  correlationId: string;
  eventSeqFirst?: number;
  eventSeqLast?: number;
  outcomeCode: string;
  reasonCode?: string;
}
//#endregion 🔖️Admin
