import { fetchWithTimeout } from "@semio-tech/framework";
import type { PersistenceBinding } from "../../../../../🟦️";
import { decodeDocumentArchiveBytes, DOCUMENT_ARCHIVE_MAXIMUM_BYTES } from "../../../../../🟦️";
import { FOLDER_ENDPOINT_PATH, FOLDER_FETCH_TIMEOUT_MS } from "../../⚙️constants/🟦️.ts";
import type { ArtifactState } from "../../🧾️document-state/🟦️.ts";
import { retireFolderCanonicalBootstrapMirror } from "../../🌉️wire-bridge/🟦️.ts";
import { setStatus } from "../../🎭️actor-lifecycle/🟦️.ts";
import { equalByteArrays } from "../../📮️hub-outbox/🟦️.ts";

/** ✍️ Folder envelope writes: the envelope URL, the retirement of the held canonical bootstrap mirror and the archive write that every folder lane (watch, config facet, lifecycle) shares. */
export function folderEnvelopeUrl(binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>, documentId: string): string {
  return `${FOLDER_ENDPOINT_PATH}?uri=${encodeURIComponent(`folder://${binding.path}`)}&documentId=${encodeURIComponent(documentId)}`;
}

export async function retireCurrentFolderCanonicalBootstrapMirror(state: ArtifactState): Promise<void> {
  const owner = state.canonicalFolderMirror;
  if (!owner) return;
  if (!(await retireFolderCanonicalBootstrapMirror(owner))) throw new Error("folder canonical bootstrap retirement failed");
  if (state.canonicalFolderMirror === owner) state.canonicalFolderMirror = null;
}

export async function writeFolder(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>, archive: readonly number[]): Promise<void> {
  await retireCurrentFolderCanonicalBootstrapMirror(state);
  const bytes = Uint8Array.from(archive);
  if (bytes.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document archive exceeds its fixed byte authority");
  decodeDocumentArchiveBytes(bytes);
  if (state.folderArchive !== null && equalByteArrays(state.folderArchive, bytes)) return;
  const response = await fetchWithTimeout(
    folderEnvelopeUrl(binding, state.config.documentId),
    { method: "PUT", headers: { "content-type": "application/octet-stream" }, body: bytes },
    { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal },
  );
  if (!response.ok) throw new Error(`folder backbone write failed (${response.status})`);
  state.folderArchive = bytes;
  state.folderBootstrapped = true;
  setStatus(state, { persisted: true });
}
