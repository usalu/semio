import type { ArtifactActorConfig } from "../../../../🟦️";
import { parseDocumentBackboneMessage } from "../../../../🟦️";
import type { MutationEnvelope } from "@semio-tech/framework-replication";
import { latestWins } from "@semio-tech/framework";
import { documentRuntimeKeyForConfig, hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";
import { newArtifactState, retireArtifactBeforeReplacement } from "../♻️artifact-lifecycle/🟦️.ts";
import { artifactScope, artifacts, emitEvent, post } from "../🧾️document-state/🟦️.ts";
import { folderBinding } from "../🎭️actor-lifecycle/🟦️.ts";
import { pollFolderOnce, watchFolder } from "../📁️folder/🟦️.ts";
import { documentSocketGrantTestIssue } from "../🔐️hub-session/🟦️.ts";
import { connectHub } from "../🔌️hub-connection/🟦️.ts";

export function openArtifact(request: ArtifactActorConfig & { readonly clientInstanceId?: string }): void {
  const config: ArtifactActorConfig = request;
  const hub = hubBinding(config);
  const runtimeKey = documentRuntimeKeyForConfig(config);
  retireArtifactBeforeReplacement(runtimeKey);
  const channel = new BroadcastChannel(`semio-doc-${runtimeKey}`);
  const state = newArtifactState(config, runtimeKey, channel, request.clientInstanceId ?? crypto.randomUUID());
  artifacts.set(runtimeKey, state);
  channel.onmessage = (messageEvent) => {
    const value = messageEvent.data;
    if (typeof value === "object" && value !== null && !Array.isArray(value) && (value as { kind?: unknown }).kind === "documentBackbone" && (value as { message?: unknown }).message instanceof Uint8Array) {
      const parsed = parseDocumentBackboneMessage((value as { message: Uint8Array }).message);
      if (parsed.envelopes.length > 0 && parsed.envelopes.every((envelope) => envelope.document_id === state.config.documentId)) emitEvent(state, { kind: "documentBackbone", message: parsed.message });
      return;
    }
    const envelopes = value as MutationEnvelope[];
    if (Array.isArray(envelopes) && envelopes.length > 0) emitEvent(state, { kind: "remoteMutations", envelopes });
  };
  const folder = folderBinding(config);
  if (folder) {
    // 🥇️ One single-flight guard per document (finding 1), shared by every trigger source.
    state.revalidateFolder = latestWins(() => pollFolderOnce(state, folder));
    if (config.watchExternal !== false) watchFolder(state, folder);
    else void state.revalidateFolder();
  }
  if (hub && hub.requestedSurfaceId === undefined && hub.installedTarget === undefined && documentSocketGrantTestIssue === null) {
    const scope = artifactScope(state);
    post({ kind: "socket-actor-failed", documentId: config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), code: "installed-target-unavailable" });
  } else if (hub) {
    connectHub(state, hub);
  }
  emitEvent(state, { kind: "status", ...state.status });
}
