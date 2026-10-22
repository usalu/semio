/** 🔌️ The socket actor readiness and failure messages of the backbone worker.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useBackboneWorker}: each function is one message arm of the backbone worker `onmessage` ladder, moved verbatim. */
import { type BackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { publishLocalPresenceActorV1 } from "../../../👕️canvas-presence/🟦️.ts";
import { scopeRuntimeKey } from "../../🔐️identity-binding/🟦️.ts";
import type { ShellCoreAfterIdentityAndDocumentPorts } from "../../🔒️identity-document-ports/🟦️.ts";
import type { RefObject } from "../../../⚛️react-port/🟦️.ts";

/** 🧦️ Resolves the waiter of a ready socket actor and publishes its local presence actor. */
export function handleSocketActorMessage({ openDocumentSessionsRef, message, socketActorReadyRef }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "openDocumentSessionsRef"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "socket-actor" }>>;
  readonly socketActorReadyRef: RefObject<Map<string, { readonly clientInstanceId: string; resolve(actorId: string): void; reject(error: Error): void }>>;
}) {
  const runtimeKey = scopeRuntimeKey(message);
  if (runtimeKey === null) return;
  const entry = openDocumentSessionsRef.current.get(runtimeKey);
  const waiter = socketActorReadyRef.current.get(runtimeKey);
  if (entry?.clientInstanceId !== message.clientInstanceId || waiter?.clientInstanceId !== message.clientInstanceId) return;
  waiter.resolve(message.actorId);
  publishLocalPresenceActorV1(runtimeKey, message.actorId);
  if (socketActorReadyRef.current.get(runtimeKey)?.clientInstanceId === message.clientInstanceId) socketActorReadyRef.current.delete(runtimeKey);
  return;
}

/** 💥️ Rejects the waiter of a socket actor that failed to start. */
export function handleSocketActorFailedMessage({ openDocumentSessionsRef, message, socketActorReadyRef }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "openDocumentSessionsRef"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "socket-actor-failed" }>>;
  readonly socketActorReadyRef: RefObject<Map<string, { readonly clientInstanceId: string; resolve(actorId: string): void; reject(error: Error): void }>>;
}) {
  const runtimeKey = scopeRuntimeKey(message);
  if (runtimeKey === null) return;
  const entry = openDocumentSessionsRef.current.get(runtimeKey);
  const waiter = socketActorReadyRef.current.get(runtimeKey);
  if (entry?.clientInstanceId !== message.clientInstanceId || waiter?.clientInstanceId !== message.clientInstanceId) return;
  waiter.reject(new Error(`socket actor unavailable (${message.code})`));
  if (socketActorReadyRef.current.get(runtimeKey)?.clientInstanceId === message.clientInstanceId) socketActorReadyRef.current.delete(runtimeKey);
  return;
}
