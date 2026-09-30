import { parseGisMapInferenceApprovalReceiptV1, parseGisMapInferenceEventPageV1, parseGisMapInferenceJobReceiptV1 } from "../../../📇️directory/🧬️schema/🟦️.ts";

function closed(value: unknown, keys: readonly string[]): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("inference-reconcile.invalid-payload");
  const row = value as Record<string, unknown>;
  if (keys.some((key) => !Object.hasOwn(row, key)) || Object.keys(row).some((key) => !keys.includes(key))) throw new Error("inference-reconcile.invalid-payload");
  return row;
}

/** 🧭️ The inference port validates its payload using its existing receipt and event contracts. */
export function parseInferenceReconcilePayloadV1(value: unknown) {
  const row = closed(value, ["receipt", "page", "approval"]);
  const receipt = parseGisMapInferenceJobReceiptV1(row.receipt);
  const rawPage = closed(row.page, ["jobId", "state", "proposalState", "cancelRequested", "expired", "proposalHash", "events", "progress", "nextCursor"]);
  if (typeof rawPage.expired !== "boolean") throw new Error("inference-reconcile.invalid-expiry");
  const { expired, ...fields } = rawPage;
  const { schema: _schema, stale: _stale, ...page } = parseGisMapInferenceEventPageV1({ ...fields, schema: "semio.hub.inference-job-events/v1", stale: fields.proposalState === "stale" });
  if (receipt.jobId !== page.jobId || receipt.state !== page.state || receipt.proposalState !== page.proposalState || receipt.proposalHash !== page.proposalHash || receipt.cursor !== page.nextCursor) throw new Error("inference-reconcile.different-job");
  let approval: { state: "available" | "undo-prepared" | "undone"; receipt: ReturnType<typeof parseGisMapInferenceApprovalReceiptV1> | null } | null = null;
  if (row.approval !== null) {
    const raw = closed(row.approval, ["state", "receipt"]);
    if (raw.state !== "available" && raw.state !== "undo-prepared" && raw.state !== "undone") throw new Error("inference-reconcile.invalid-approval");
    const approved = raw.state === "available" ? parseGisMapInferenceApprovalReceiptV1(raw.receipt) : null;
    if (approved === null ? raw.receipt !== null : approved.jobId !== page.jobId || !approved.applied || approved.proposalHash !== page.proposalHash) throw new Error("inference-reconcile.invalid-approval-receipt");
    approval = { state: raw.state, receipt: approved };
  }
  if ((page.proposalState === "approved") !== (approval !== null) || (approval !== null && page.proposalHash === null)) throw new Error("inference-reconcile.invalid-approval-state");
  return { receipt, page: { ...page, expired }, approval };
}
