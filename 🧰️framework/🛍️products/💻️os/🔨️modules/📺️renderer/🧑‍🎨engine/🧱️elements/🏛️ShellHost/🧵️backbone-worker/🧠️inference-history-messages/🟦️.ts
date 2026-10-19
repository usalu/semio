/** 🧠️ The inference history status message of the backbone worker.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useBackboneWorker}: each function is one message arm of the backbone worker `onmessage` ladder, moved verbatim. */
import { directorySessionAuthorityIsCurrentV1 } from "@semio-tech/framework/directory/session-refresh";
import { type BackboneWorkerResponse, documentRuntimeKeyV1 } from "@semio-tech/framework/backbone";
import type { ShellCoreAfterIdentityAndDocumentPorts } from "../../🔒️identity-document-ports/🟦️.ts";

/** 📜️ Folds an inference history status into the ui state of its open document. */
export function handleInferenceHistoryStatusMessage({ openDocumentSessionsRef, verifiedSessionAuthorityRef, retiredSessionDocumentOwnersRef, setInferenceHistoryByRuntimeKey, historyOrderRef, message }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "openDocumentSessionsRef" | "verifiedSessionAuthorityRef" | "retiredSessionDocumentOwnersRef" | "setInferenceHistoryByRuntimeKey" | "historyOrderRef"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "inference-history-status" }>>;
}) {
  const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...message.scope });
  const entry = openDocumentSessionsRef.current.get(runtimeKey);
  const authority = verifiedSessionAuthorityRef.current;
  if (authority === null || !directorySessionAuthorityIsCurrentV1(authority, authority) || entry === undefined || retiredSessionDocumentOwnersRef.current.has(entry) || entry.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.scope.spaceId || entry.scope.documentId !== message.scope.documentId) return;
  setInferenceHistoryByRuntimeKey((current) => {
    if (!directorySessionAuthorityIsCurrentV1(authority, verifiedSessionAuthorityRef.current) || retiredSessionDocumentOwnersRef.current.has(entry)) return current;
    const previous = current[runtimeKey];
    if (previous !== undefined && (message.historyEpoch < previous.historyEpoch || previous.clientInstanceId !== message.clientInstanceId)) return current;
    const order = previous?.historyEpoch === message.historyEpoch ? previous.order : ++historyOrderRef.current;
    return { ...current, [runtimeKey]: { historyEpoch: message.historyEpoch, clientInstanceId: message.clientInstanceId, scope: message.scope, sessionInstanceId: entry.session.instanceId, status: message.status, owner: message.owner, serviceId: message.serviceId, order, authority } };
  });
  return;
}
