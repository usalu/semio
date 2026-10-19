/** 🥾️ The artifact bootstrap progress, failure and rebootstrap messages of the backbone worker.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useBackboneWorker}: each function is one message arm of the backbone worker `onmessage` ladder, moved verbatim. */
import { shellDialogSessionIsCurrentV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { type BackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { reduceBootstrapUiState, type BootstrapUiState } from "../../🪪️host-bootstrap/🟦️.tsx";
import { scopeRuntimeKey } from "../../🔐️identity-binding/🟦️.ts";
import type { ShellCoreAfterIdentityAndDocumentPorts } from "../../🔒️identity-document-ports/🟦️.ts";
import type { Dispatch, RefObject, SetStateAction } from "react";

/** 🚚️ Folds bootstrap progress, failure and rebootstrap requests into the document ui state. */
export function handleArtifactBootstrapMessage({ openDocumentSessionsRef, failDocumentBackbone, retireBrowserActorUi, cancelSpaceArtifactCreationsForRuntime, sessionRef, message, setBootstrapUiByDocument, worker, rebootstrapDiscardedSessionsRef }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "openDocumentSessionsRef" | "failDocumentBackbone" | "retireBrowserActorUi" | "cancelSpaceArtifactCreationsForRuntime" | "sessionRef"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "artifact-bootstrap-progress" }>> | Extract<BackboneWorkerResponse, Readonly<{ kind: "artifact-bootstrap-failed" }>> | Extract<BackboneWorkerResponse, Readonly<{ kind: "artifact-rebootstrap-required" }>>;
  readonly setBootstrapUiByDocument: Dispatch<SetStateAction<BootstrapUiState>>;
  readonly worker: Worker;
  readonly rebootstrapDiscardedSessionsRef: RefObject<Map<string, ActiveSession>>;
}) {
  const runtimeKey = scopeRuntimeKey(message);
  if (runtimeKey === null) return;
  const entry = openDocumentSessionsRef.current.get(runtimeKey);
  if (!entry || entry.clientInstanceId !== message.clientInstanceId) return;
  setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, message));
  if (entry.creationMount !== null && message.kind !== "artifact-bootstrap-progress") {
    failDocumentBackbone(runtimeKey, entry, new Error(message.kind === "artifact-bootstrap-failed" ? message.message : "artifact-creation.rebootstrap-required"));
    return;
  }
  if (message.kind === "artifact-bootstrap-failed") {
    retireBrowserActorUi(runtimeKey, "browser-actor-action: bootstrap failed");
    entry.replacements.invalidate();
    entry.rejectReady(new Error(message.message));
    const retirement = entry.port?.retire();
    void retirement?.catch(error => failDocumentBackbone(runtimeKey, entry, error));
    entry.port = null;
    entry.pending = [];
    entry.pendingBytes = 0;
  }
  if (message.kind === "artifact-rebootstrap-required") {
    entry.replacements.invalidate();
    const retirement = entry.port?.retire();
    void retirement?.catch(error => failDocumentBackbone(runtimeKey, entry, error));
    entry.port = null;
    entry.pending = [];
    entry.pendingBytes = 0;
    cancelSpaceArtifactCreationsForRuntime(runtimeKey, worker);
    retireBrowserActorUi(runtimeKey, "browser-actor-action: rebootstrap required");
    const active = sessionRef.current;
    if (shellDialogSessionIsCurrentV1(active, entry.session)) {
      rebootstrapDiscardedSessionsRef.current.set(runtimeKey, entry.session);
    }
  }
  return;
}
