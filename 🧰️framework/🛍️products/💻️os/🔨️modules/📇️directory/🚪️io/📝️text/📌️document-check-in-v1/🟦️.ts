/** 🚪️ Canonical Directory JSON admission and physical receipt projection. */
import { documentCheckInStatusFromValueV1, type DocumentCheckInStatusV1, type DocumentCheckInV1, DOCUMENT_CHECK_IN_MAX_BYTES, DOCUMENT_CHECK_IN_SCHEMA_V1, DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1, PHASES, REFUSALS, canonicalFrontier, encoder, exactKeys, isEditedArtifactFrontierV1, natural, ready, record, requestId } from "../../../🧬️schema/📌️document-check-in-v1/🟦️.ts";

/** 📤️ The canonical request body; throws for a request the hub would refuse unread. */
export function documentCheckInCanonicalJson(request: DocumentCheckInV1): string {
  const canonical = { schema: request.schema, requestId: request.requestId, head: canonicalFrontier(request.head) };
  if (canonical.schema !== DOCUMENT_CHECK_IN_SCHEMA_V1 || !requestId(canonical.requestId) || !isEditedArtifactFrontierV1(canonical.head)) throw new Error("document-check-in.invalid-request");
  const source = JSON.stringify(canonical);
  if (encoder.encode(source).length > DOCUMENT_CHECK_IN_MAX_BYTES) throw new Error("document-check-in.request-too-large");
  return source;
}
/** 📥️ One exact canonical request; `null` for padding, reordering, overposting or bounds. */
export function parseDocumentCheckInV1(source: string): DocumentCheckInV1 | null {
  if (encoder.encode(source).length > DOCUMENT_CHECK_IN_MAX_BYTES) return null;
  let parsed: unknown;
  try {
    parsed = JSON.parse(source);
  } catch {
    return null;
  }
  const value = record(parsed);
  if (value === null || !exactKeys(value, ["schema", "requestId", "head"]) || value.schema !== DOCUMENT_CHECK_IN_SCHEMA_V1 || !requestId(value.requestId) || !isEditedArtifactFrontierV1(value.head)) return null;
  const request = value as DocumentCheckInV1;
  return documentCheckInCanonicalJson(request) === source ? request : null;
}
function canonicalStatus(status: DocumentCheckInStatusV1): string {
  return JSON.stringify({
    schema: status.schema,
    requestId: status.requestId,
    phase: status.phase,
    progress: { completedUnits: status.progress.completedUnits, totalUnits: status.progress.totalUnits },
    ...(status.ready === undefined ? {} : { ready: { checkpointId: status.ready.checkpointId, parentCheckpointId: status.ready.parentCheckpointId, baseline: canonicalFrontier(status.ready.baseline) } }),
    ...(status.refusal === undefined ? {} : { refusal: status.refusal }),
  });
}
/** 🧾️ One exact hub status: only ready names a checkpoint, only failed a refusal, progress never overruns. */
export function parseDocumentCheckInStatusV1(source: string): DocumentCheckInStatusV1 | null {
  if (encoder.encode(source).length > DOCUMENT_CHECK_IN_MAX_BYTES) return null;
  let parsed: unknown;
  try {
    parsed = JSON.parse(source);
  } catch {
    return null;
  }
  const status = documentCheckInStatusFromValueV1(parsed);
  return status !== null && canonicalStatus(status) === source ? status : null;
}
